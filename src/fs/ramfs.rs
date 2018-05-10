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

use super::{Fs, FsRef, FS};
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
	pub fn new(name: &str) -> FsRef {
		return FS.emplace(Mutex::new(Box::new(RamFs {
			name: String::from(name),
			root: Node::new(),
		}))).1;
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
