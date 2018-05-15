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

use fs::FsRef;
use util::uid::WeakTracker;
use spin::Mutex;
use alloc::{
	arc::Arc,
	string::String,
	BTreeMap,
};

pub struct Node {
	mount: Option<FsRef>,
	children: BTreeMap<String, NodeRef>,
}

pub type NodeRef = Arc<Mutex<Node>>;

lazy_static! {
	static ref NODES: WeakTracker<Mutex<Node>> = WeakTracker::new();
}

impl<'a> Node {
	/// Create a new node, returning a corresponding NodeRef
	pub fn new() -> NodeRef {
		return NODES.emplace(Mutex::new(Node {
			mount: None,
			children: BTreeMap::new(),
		})).1;
	}

	/// Follow a NodeRef through to its mount location
	pub fn follow(node: NodeRef) -> NodeRef {
		match node.lock().mount {
			Some(ref fs) => fs.lock().root(),
			None => node.clone(),
		}
	}

	/// Add a NodeRef to the node
	pub fn add(&mut self, name: &str, child: &Arc<Mutex<Node>>) -> NodeRef {
		match self.get(name) {
			Some(n) => return n.clone(),
			None => {},
		}
		self.children_mut().insert(String::from(name), child.clone());
		return child.clone()
	}

	pub fn get(&'a self, name: &str) -> Option<&'a NodeRef> {
		self.children().get(name)
	}

	/// Mount a filesystem on this node
	pub fn mount(&mut self, fs: FsRef) {
		self.mount = Some(fs);
	}

	/// Return an iterable map to the node's children, with their respective names as the keys
	pub fn children(&'a self) -> &'a BTreeMap<String, NodeRef> {
		&self.children
	}

	/// Return a mutable iterable map to the node's children, with their respective names as the keys
	pub fn children_mut(&'a mut self) -> &'a mut BTreeMap<String, NodeRef> {
		&mut self.children
	}
}
