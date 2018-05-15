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

use arch::tags::BootData;
use util::Tar;
use fs::{FS, FsRef, RamFs};
use spin::Mutex;

lazy_static! {
	static ref ROOTFS: Mutex<Option<FsRef>> = Mutex::new(None);
}

pub fn init(boot_data: &BootData) {
	for module in &boot_data.modules {
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
	match *ROOTFS.lock() {
		Some(ref rootfs) => display_node(&rootfs.lock().root().clone(), "/", 0),
		None => panic!("No root filesystem"),
	}

	logln!("Filesystems:");
	for (uid, item) in FS.items().lock().iter() {
		logln!("Filesystem (uid = {}, name = {})", uid, item.lock().name());
	}
}
