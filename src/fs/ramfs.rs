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

use super::{InodeId, Fs, FsErr, FileType};
use alloc::{
	String,
	BTreeMap,
	Vec,
	arc::Arc,
	boxed::Box,
};
use bimap::BiMap;
use spin::Mutex;

enum InodeData {
	Regular { data: Vec<u8> },
	Directory { children: BiMap<String, InodeId> },
	Mount { mount: Arc<Box<Fs>> },
}

struct Inode {
	data: InodeData,
}

struct RamFsData {
	name: String,
	inode_counter: InodeId,
	inodes: BTreeMap<InodeId, Inode>,
	root: InodeId,
}

impl RamFsData {
	fn create_inode(&mut self, ft: FileType) -> InodeId {
		self.inode_counter += 1;
		let new_id = self.inode_counter;
		self.inodes.insert(new_id, Inode {
			data: match ft {
				_ => InodeData::Directory { children: BiMap::new() }
			},
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

		let root_id = ramfs.data.lock().create_inode(FileType::Directory);
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

	fn get_data(&self, inode: InodeId) -> Result<Vec<u8>, FsErr> {
		match self.data.lock().inodes.get(&inode) {
			Some(i) => match i.data {
				InodeData::Regular { ref data } => Ok(data.clone()),
				_ => Err(FsErr::InvalidFileType),
			},
			None => Err(FsErr::NoSuchFile),
		}
	}

	fn set_data(&self, inode: InodeId, new_data: Vec<u8>) -> Result<(), FsErr> {
		match self.data.lock().inodes.get_mut(&inode) {
			Some(i) => match i.data {
				InodeData::Regular { ref mut data } => { *data = new_data.clone(); Ok(()) },
				_ => Err(FsErr::InvalidFileType),
			},
			None => Err(FsErr::NoSuchFile),
		}
	}

	fn children(&self, inode: InodeId) -> Result<Vec<(String, InodeId)>, FsErr> {
		match self.data.lock().inodes.get(&inode) {
			Some(i) => match i.data {
				InodeData::Directory { ref children } => Ok(
					children
					.iter()
					.map(|d| (d.0.clone(), *d.1))
					.collect()
				),
				_ => Err(FsErr::InvalidFileType),
			},
			None => Err(FsErr::NoSuchFile),
		}
	}

	fn child_ids(&self, inode: InodeId) -> Result<Vec<InodeId>, FsErr> {
		match self.data.lock().inodes.get(&inode) {
			Some(i) => match i.data {
				InodeData::Directory { ref children } => Ok(
					children
					.right_values()
					.map(|i| *i)
					.collect()
				),
				_ => Err(FsErr::InvalidFileType),
			},
			None => Err(FsErr::NoSuchFile),
		}
	}

	fn child_names(&self, inode: InodeId) -> Result<Vec<String>, FsErr> {
		match self.data.lock().inodes.get(&inode) {
			Some(i) => match i.data {
				InodeData::Directory { ref children } => Ok(
					children
					.left_values()
					.map(|s| s.clone())
					.collect()
				),
				_ => Err(FsErr::InvalidFileType),
			},
			None => Err(FsErr::NoSuchFile),
		}
	}

	fn get_child_id(&self, inode: InodeId, name: &str) -> Result<InodeId, FsErr> {
		match self.data.lock().inodes.get(&inode) {
			Some(i) => match i.data {
				InodeData::Directory { ref children } => match children.get_by_left(&String::from(name)) {
					Some(i) => Ok(*i),
					None => Err(FsErr::NoSuchChild),
				},
				_ => Err(FsErr::InvalidFileType),
			},
			None => Err(FsErr::NoSuchFile),
		}
	}

	fn get_child_name(&self, inode: InodeId, id: InodeId) -> Result<String, FsErr> {
		match self.data.lock().inodes.get(&inode) {
			Some(i) => match i.data {
				InodeData::Directory { ref children } => match children.get_by_right(&id) {
					Some(s) => Ok(s.clone()),
					None => Err(FsErr::NoSuchChild),
				},
				_ => Err(FsErr::InvalidFileType),
			},
			None => Err(FsErr::NoSuchFile),
		}
	}

	fn add_child(&self, inode: InodeId, name: &str, ft: FileType) -> Result<InodeId, FsErr> {
		// TODO: Clean this up. We check twice to make the borrow checker happy
		let mut data = self.data.lock();
		if !data.inodes.contains_key(&inode) {
			return Err(FsErr::NoSuchFile);
		}

		// TODO: Make this not a directory
		let new_id = data.create_inode(ft);
		if let Some(i) = data.inodes.get_mut(&inode) {
			match i.data {
				InodeData::Directory { ref mut children } => children.insert(String::from(name), new_id),
				_ => return Err(FsErr::InvalidFileType),
			};
		}

		Ok(new_id)
	}

	fn get_mount(&self, inode: InodeId) -> Result<Arc<Box<Fs>>, FsErr> {
		match self.data.lock().inodes.get(&inode) {
			Some(i) => match i.data {
				InodeData::Mount { ref mount } => Ok(mount.clone()),
				_ => Err(FsErr::InvalidFileType),
			},
			None => Err(FsErr::NoSuchFile),
		}
	}

	fn mount(&self, inode: InodeId, fs: &Arc<Box<Fs>>) -> Result<(), FsErr> {
		match self.data.lock().inodes.get_mut(&inode) {
			Some(ref mut i) => match i.data {
				InodeData::Mount { ref mut mount } => { *mount = fs.clone(); Ok(()) },
				_ => Err(FsErr::InvalidFileType),
			},
			None => Err(FsErr::NoSuchFile),
		}
	}
}
