// file : ramfs.rs
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

use super::{InodeId, Fs, FsErr};
use alloc::{
	String,
	BTreeMap,
	Vec,
	arc::Arc,
	boxed::Box,
};
use bimap::BiMap;
use spin::Mutex;

struct Inode {
	mount: Option<Arc<Box<Fs>>>,
	children: BiMap<String, InodeId>,
	data: Vec<u8>,
}

struct RamFsData {
	name: String,
	inode_counter: InodeId,
	inodes: BTreeMap<InodeId, Inode>,
	root: InodeId,
}

impl RamFsData {
	fn create_inode(&mut self) -> InodeId {
		self.inode_counter += 1;
		let new_id = self.inode_counter;
		self.inodes.insert(new_id, Inode {
			mount: None,
			children: BiMap::new(),
			data: Vec::new(),
		});
		return new_id;
	}
}

pub struct RamFs {
	data: Mutex<RamFsData>,
}

impl RamFs {
	pub fn new(name: &str) -> RamFs {
		let ramfs = RamFs {
			data: Mutex::new(RamFsData {
				name: String::from(name),
				inode_counter: 0,
				inodes: BTreeMap::new(),
				root: 0,
			}),
		};

		let root_id = ramfs.data.lock().create_inode();
		ramfs.data.lock().root = root_id;
		return ramfs;
	}
}

impl Fs for RamFs {
	fn name(&self) -> String {
		self.data.lock().name.clone()
	}

	fn root_id(&self) -> InodeId {
		self.data.lock().root
	}

	fn children(&self, inode: InodeId) -> Result<Vec<(String, InodeId)>, FsErr> {
		match self.data.lock().inodes.get(&inode) {
			Some(i) => Ok(
				i.children
				.iter()
				.map(|d| (d.0.clone(), *d.1))
				.collect()),
			None => Err(FsErr::NoSuchFile),
		}
	}

	fn child_ids(&self, inode: InodeId) -> Result<Vec<InodeId>, FsErr> {
		match self.data.lock().inodes.get(&inode) {
			Some(i) => Ok(
				i.children
				.right_values()
				.map(|i| *i)
				.collect()),
			None => Err(FsErr::NoSuchFile),
		}
	}

	fn child_names(&self, inode: InodeId) -> Result<Vec<String>, FsErr> {
		match self.data.lock().inodes.get(&inode) {
			Some(i) => Ok(
				i.children
				.left_values()
				.map(|s| s.clone())
				.collect()),
			None => Err(FsErr::NoSuchFile),
		}
	}

	fn get_child_id(&self, inode: InodeId, name: &str) -> Result<InodeId, FsErr> {
		match self.data.lock().inodes.get(&inode) {
			Some(i) => match i.children.get_by_left(&String::from(name)) {
				Some(i) => Ok(*i),
				None => Err(FsErr::NoSuchChild),
			},
			None => Err(FsErr::NoSuchFile),
		}
	}

	fn get_child_name(&self, inode: InodeId, id: InodeId) -> Result<String, FsErr> {
		match self.data.lock().inodes.get(&inode) {
			Some(i) => match i.children.get_by_right(&id) {
				Some(n) => Ok(n.clone()),
				None => Err(FsErr::NoSuchChild),
			},
			None => Err(FsErr::NoSuchFile),
		}
	}

	fn add_child(&self, inode: InodeId, name: &str) -> Result<InodeId, FsErr> {
		// TODO: Clean this up. We check twice to make the borrow checker happy
		let mut data = self.data.lock();
		if !data.inodes.contains_key(&inode) {
			return Err(FsErr::NoSuchFile);
		}

		let new_id = data.create_inode();
		if let Some(i) = data.inodes.get_mut(&inode) {
			i.children.insert(String::from(name), new_id);
		}

		Ok(new_id)
	}

	fn get_mount(&self, inode: InodeId) -> Result<Arc<Box<Fs>>, FsErr> {
		match self.data.lock().inodes.get(&inode) {
			Some(i) => match i.mount {
				Some(ref m) => Ok(m.clone()),
				None => Err(FsErr::NoSuchMount),
			},
			None => Err(FsErr::NoSuchFile),
		}
	}

	fn mount(&self, inode: InodeId, fs: &Arc<Box<Fs>>) -> Result<(), FsErr> {
		match self.data.lock().inodes.get_mut(&inode) {
			Some(ref mut i) => match i.mount.is_some() {
				true => Err(FsErr::MountPointInUse),
				false => { i.mount = Some(fs.clone()); Ok(()) },
			},
			None => Err(FsErr::NoSuchFile),
		}
	}
}
