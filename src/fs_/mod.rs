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
}

pub trait Fs: Send + Sync {
	fn root_id(&self) -> InodeId;
	fn children(&self, parent: InodeId) -> Result<Vec<(String, InodeId)>, FsErr>;
	fn child_ids(&self, parent: InodeId) -> Result<Vec<InodeId>, FsErr>;
	fn child_names(&self, parent: InodeId) -> Result<Vec<String>, FsErr>;
	fn get_child_id(&self, parent: InodeId, name: &str) -> Result<InodeId, FsErr>;
	fn get_child_name(&self, parent: InodeId, id: InodeId) -> Result<String, FsErr>;
	fn add_child(&self, parent: InodeId, name: &str) -> Result<InodeId, FsErr>;
	fn mount(&self, parent: InodeId, name: Arc<Box>) -> Result<InodeId, FsErr>;
}

lazy_static! {
	static ref ROOT_FS: Mutex<Arc<Box<Fs>>> = Mutex::new(Arc::new(Box::new(RamFs::new())));
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
}

fn display_tree(file: &File, name: &str, depth: usize) {
	logln!("{}", name);

	if depth > 32 {
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
		log!("|- ");
		display_tree(file, name.as_str(), depth + 1);
	}
}

pub fn init() {
	let root = File::from("/").unwrap();
	let bin = root.add_child("bin").unwrap();
	let dev = root.add_child("dev").unwrap();
	let home = root.add_child("home").unwrap();
	let sys = root.add_child("sys").unwrap();

	let test = home.add_child("test");

	display_tree(&root, "/", 0);
}
