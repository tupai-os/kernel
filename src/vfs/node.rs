// file : node.rs
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

use util::uid::{Tracker};
use spin::Mutex;
use alloc::{
	arc::Arc,
	string::String,
	BTreeMap,
};

pub struct Node {
	children: BTreeMap<String, Arc<Mutex<Node>>>,
}

lazy_static! {
	static ref NODES: Tracker<Mutex<Node>> = Tracker::new();
}

impl<'a> Node {
	pub fn new() -> Arc<Mutex<Node>> {
		return NODES.emplace(Mutex::new(Node {
			children: BTreeMap::new(),
		})).1;
	}

	pub fn add_child(&mut self, name: &str, child: &Arc<Mutex<Node>>) -> Option<Arc<Mutex<Node>>> {
		return match self.children.get(name) {
			Some(_) => None,
			None => {
				self.children.insert(String::from(name), child.clone());
				Some(child.clone())
			}
		}
	}

	pub fn children(&'a self) -> &'a BTreeMap<String, Arc<Mutex<Node>>> {
		&self.children
	}
}
