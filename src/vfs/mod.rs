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

mod node;

// Reexports
pub use self::node::Node as Node;
pub use self::node::NodeRef as NodeRef;

// *****************************************************
// * IMPORTANT: Filesystem operations are not IRQ-safe *
// *****************************************************

use util::bootcfg::BootCfg;
use util::Tar;
use fs::{FS, FsHandle, RamFs};
use spin::Mutex;

lazy_static! {
	static ref ROOTFS: Mutex<Option<FsHandle>> = Mutex::new(None);
}

pub fn init(bootcfg: &BootCfg) {
	for module in &bootcfg.modules {
		loginfo!("Handling module: (args = {:?})", module.args);

		let tar = unsafe { Tar::from(module.start) };

		// We found the rootfs
		if module.args.contains(&"rootfs") {
			*ROOTFS.lock() = Some(RamFs::from_tar("rootfs", tar));
		}
	}

logok!("VFS initiated");
}

fn display_node(node: &NodeRef, name: &str, depth: usize) {
	logln!("{}", name);

	if depth > 32 {
		logln!("Maximum recursion depth reached");
		return;
	}

	for (name, node) in node.lock().children().iter() {
		log!("|-");
		for _i in 0..depth {
			log!("--");
		}
		log!(" ");
		display_node(&node, name.as_str(), depth + 1);
	}
}

pub fn display() {
	// TODO: Remove unwraps
	match *ROOTFS.lock() {
		Some(ref rootfs) => display_node(&rootfs.root().unwrap().clone(), "/", 0),
		None => panic!("No root filesystem"),
	}

	logln!("Filesystems:");
	for (uid, item) in FS.items().lock().iter() {
		logln!("Filesystem (uid = {}, name = {})", uid, item.lock().name());
	}
}
