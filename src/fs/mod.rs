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

use vfs::NodeRef;
use util::uid::Tracker;
use spin::Mutex;
use alloc::{
	String,
	arc::Arc,
	boxed::Box,
};

pub trait Fs: Send {
	fn name(&self) -> String;
	fn root(&self) -> NodeRef;
}

pub type FsRef = Arc<Mutex<Box<Fs>>>;

lazy_static! {
	pub static ref FS: Tracker<Mutex<Box<Fs>>> = Tracker::new();
}

pub fn init() {
	logok!("Initiated filesystems");
}
