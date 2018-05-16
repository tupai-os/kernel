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

use super::{Fs, FsHandle, FS};
use util::{Tar, Path};
use vfs::{Node, NodeRef};
use spin::Mutex;
use alloc::{
	String,
	boxed::Box
};

pub struct RamFs {
	name: String,
	root: NodeRef,
}

impl RamFs {
	pub fn new(name: &str) -> FsHandle {
		return FsHandle::from_uid(FS.emplace(Mutex::new(Box::new(RamFs {
			name: String::from(name),
			root: Node::new(),
		}))).0);
	}

	pub fn from_tar(name: &str, tar: Tar) -> FsHandle {
		let fs = RamFs::new(name);

		for file in tar {
			if file.size() < 100 {
				logln!("Data in file {} ({} bytes):\n {}", file.name(), file.size(), String::from_utf8_lossy(file.data()));
			}

			let mut node = fs.root().unwrap();
			for part in Path::from(&file.name()) {
				// This syntax is pretty shitty. We do it to avoid deadlocks.
				let nnode;
				let val = match node.lock().get(&part) {
					Some(n) => Some(n.clone()),
					None => None,
				};
				if let Some(n) = val {
					nnode = n.clone();
				} else {
					nnode = node.lock().add(&part, &Node::new());
				}

				node = nnode;
			}
		}

		return fs;
	}
}

impl Fs for RamFs {
	fn name(&self) -> String {
		self.name.clone()
	}

	fn root(&self) -> NodeRef {
		self.root.clone()
	}
}
