// file : mod.rs
//
// Copyright (C) 2018  Joshua Barretto <joshua.s.barretto@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

mod ramfs;

// Reexports
pub use self::ramfs::RamFs as RamFs;

use util::{
	bootcfg::BootCfg,
	Tar,
	Path,
};
use alloc::{
	String,
	Vec,
	arc::{Weak, Arc},
	boxed::Box,
};
use spin::Mutex;

pub type InodeId = u64;

#[derive(Debug)]
pub enum FsErr {
	NoSuchFs,
	NoSuchFile,
	NoSuchChild,
	NoSuchMount,
	MountPointInUse,
}

pub trait Fs: Send + Sync {
	fn name(&self) -> String;
	fn root_id(&self) -> InodeId;

	fn children(&self, inode: InodeId) -> Result<Vec<(String, InodeId)>, FsErr>;
	fn child_ids(&self, inode: InodeId) -> Result<Vec<InodeId>, FsErr>;
	fn child_names(&self, inode: InodeId) -> Result<Vec<String>, FsErr>;

	fn get_child_id(&self, inode: InodeId, name: &str) -> Result<InodeId, FsErr>;
	fn get_child_name(&self, inode: InodeId, id: InodeId) -> Result<String, FsErr>;
	fn add_child(&self, inode: InodeId, name: &str) -> Result<InodeId, FsErr>;

	fn get_mount(&self, inode: InodeId) -> Result<Arc<Box<Fs>>, FsErr>;
	fn mount(&self, inode: InodeId, fs: &Arc<Box<Fs>>) -> Result<(), FsErr>;

	fn load_tar(&self, tar: Tar) -> Result<(), FsErr> {
		for file in tar {
			let mut inode = self.root_id();
			for part in Path::from(&file.name()) {
				match self.get_child_id(inode, &part) {
					Ok(i) => inode = i,
					Err(FsErr::NoSuchChild) => match self.add_child(inode, &part) {
						Ok(_) => {},
						Err(e) => { return Err(e); },
					},
					Err(e) => { return Err(e); },
				}
			}
		}
		Ok(())
	}
}

lazy_static! {
	static ref ROOT_FS: Mutex<Arc<Box<Fs>>> = Mutex::new(Arc::new(Box::new(RamFs::new(&"rootfs"))));
}

#[derive(Clone)]
pub struct File {
	fs: Weak<Box<Fs>>,
	id: InodeId,
}

impl File {
	pub fn from(path: &str) -> Result<File, FsErr> {
		let root_id = ROOT_FS.lock().root_id();
		File::from_parts(&Arc::downgrade(&ROOT_FS.lock()), root_id).trace(path)
	}

	pub fn root(fs: &Arc<Box<Fs>>) -> File {
		File::from_parts(&Arc::downgrade(fs), fs.root_id())
	}

	fn from_parts(fs: &Weak<Box<Fs>>, id: InodeId) -> File {
		File {
			fs: fs.clone(),
			id,
		}
	}

	pub fn trace(&self, path: &str) -> Result<File, FsErr> {
		let mut cfile = self.clone();
		for part in path.split_terminator('/') {
			if part != "" {
				match cfile.child(part) {
					Ok(f) => cfile = f,
					Err(e) => return Err(e),
				}
			}
		}
		return Ok(cfile);
	}

	pub fn child(&self, name: &str) -> Result<File, FsErr> {
		match self.fs.upgrade() {
			Some(fs) => match fs.get_child_id(self.id, name) {
				Ok(id) => Ok(File::from_parts(&self.fs, id)),
				Err(e) => Err(e),
			},
			None => Err(FsErr::NoSuchFs),
		}
	}

	// TODO: A lot of copying here. Smarten this up.
	pub fn children(&self) -> Result<Vec<(String, File)>, FsErr> {
		match self.fs.upgrade() {
			Some(fs) => match fs.children(self.id) {
				Ok(data) => Ok(data.iter().map(|d| (d.0.clone(), File::from_parts(&self.fs, d.1))).collect()),
				Err(e) => Err(e),
			},
			None => Err(FsErr::NoSuchFs),
		}
	}

	pub fn add_child(&self, name: &str) -> Result<File, FsErr> {
		match self.fs.upgrade() {
			Some(fs) => match fs.add_child(self.id, name) {
				Ok(id) => Ok(File::from_parts(&self.fs, id)),
				Err(e) => Err(e),
			},
			None => Err(FsErr::NoSuchFs),
		}
	}

	pub fn get_mount(&self) -> Result<Arc<Box<Fs>>, FsErr> {
		match self.fs.upgrade() {
			Some(fs) => fs.get_mount(self.id),
			None => Err(FsErr::NoSuchFs),
		}
	}

	pub fn mount(&self, mfs: &Arc<Box<Fs>>) -> Result<(), FsErr> {
		match self.fs.upgrade() {
			Some(fs) => fs.mount(self.id, mfs),
			None => Err(FsErr::NoSuchFs),
		}
	}
}

pub fn init(bootcfg: &BootCfg) {
	for module in &bootcfg.modules {
		loginfo!("Handling module: (args = {:?})", module.args);

		// We found the rootfs
		if module.args.contains(&"rootfs") {
			let tar = unsafe { Tar::from(module.start) };
			*ROOT_FS.lock() = Arc::new(Box::new(RamFs::new("rootfs")));
			ROOT_FS.lock().load_tar(tar);
		}
	}

	let root = File::from("/").unwrap();
	let bin = root.add_child("bin").unwrap();
	let dev = root.add_child("dev").unwrap();
	let home = root.add_child("home").unwrap();
	let sys = root.add_child("sys").unwrap();

	let test = home.add_child("test").unwrap();

	display();
}

fn display_tree(file: &File, name: &str, depth: usize) {
	logln!("{}", name);

	if depth > 12 {
		logln!("Maximum recursion depth reached");
		return;
	}

	for (name, file) in file.children().unwrap().iter() {
		for _i in 0..depth {
			log!("--");
		}
		if depth > 0 {
			log!("-");
		}

		match file.get_mount() {
			Ok(fs) => {
				log!("|- [m] ");
				display_tree(&File::root(&fs), name.as_str(), depth + 1);
			},
			Err(_) => {
				log!("|- ");
				display_tree(file, name.as_str(), depth + 1);
			}
		}
	}
}

pub fn display() {
	display_tree(&File::from("/").expect("Could not find root file"), "/", 0);
}
