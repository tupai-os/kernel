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

mod handle;

// Reexports
pub use self::handle::ProcessHandle as ProcessHandle;

// *****************************************************
// * IMPORTANT: Process operations are not IRQ-safe *
// *****************************************************

use llapi::cpu::paging::PageMap;
use util::uid::Tracker;
use thread::{ThreadHandle};
use spin::Mutex;
use alloc::{
	string::String,
	arc::Arc,
	BTreeSet,
	Vec,
};

pub struct Process {
	name: String,
	mmap: Arc<PageMap>,
	threads: BTreeSet<ThreadHandle>,
}

lazy_static! {
	static ref PROCESSES: Tracker<Mutex<Process>> = Tracker::new();
}

pub fn init() {
	// Create kernel process
	PROCESSES.emplace_with_uid(
		ProcessHandle::kernel().uid(),
		Mutex::new(Process {
			name: String::from("kernel"),
			mmap: Arc::new(PageMap::new()),
			threads: BTreeSet::new(),
		})
	);
}

// TODO: Make this more efficient
pub fn list() -> Vec<ProcessHandle> {
	return PROCESSES
		.uids()
		.into_iter()
		.map(|uid| ProcessHandle::from_uid(uid))
		.collect();
}

#[derive(Debug)]
pub enum ProcessErr {}

pub fn new(name: &str) -> Result<ProcessHandle, ProcessErr> {
	return Ok(ProcessHandle::from_uid(
		PROCESSES.emplace(Mutex::new(Process {
			name: String::from(name),
			mmap: Arc::new(PageMap::new()),
			threads: BTreeSet::new(),
		})).0
	));
}
