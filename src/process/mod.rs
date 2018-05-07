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

use llapi::mem::PageMap;
use util::uid::{Uid, Tracker};
use thread;
use thread::{ThreadHandle, ThreadErr};
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
	threads: Mutex<BTreeSet<ThreadHandle>>, // TODO: Make this IRQ-safe
}

lazy_static! {
	static ref PROCESSES: Tracker<Process> = Tracker::new();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessHandle {
	uid: Uid,
}

pub fn init() {
	let kernel = PROCESSES.emplace_with_uid(
		ProcessHandle::kernel().uid(),
		Process {
			name: String::from("kernel"),
			mmap: Arc::new(PageMap::new()),
			threads: Mutex::new(BTreeSet::new()),
		}
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
	return Ok(ProcessHandle {
		uid: PROCESSES.emplace(Process {
			name: String::from(name),
			mmap: Arc::new(PageMap::new()),
			threads: Mutex::new(BTreeSet::new()),
		}).0
	});
}

impl ProcessHandle {
	pub const fn invalid() -> ProcessHandle {
		ProcessHandle {
			uid: -2,
		}
	}

	pub const fn free() -> ProcessHandle {
		ProcessHandle {
			uid: -1,
		}
	}

	pub const fn kernel() -> ProcessHandle {
		ProcessHandle {
			uid: 0,
		}
	}

	pub const fn from_uid(uid: Uid) -> ProcessHandle {
		ProcessHandle {
			uid: uid,
		}
	}

	pub fn uid(&self) -> Uid {
		self.uid
	}

	pub fn name(&self) -> Option<String> {
		match PROCESSES.get(self.uid) {
			Some(p) => Some(p.name.clone()),
			_ => None,
		}
	}

	pub fn threads(&self) -> Option<Vec<ThreadHandle>> {
		match PROCESSES.get(self.uid) {
			Some(p) => Some(p.threads.lock().iter().cloned().collect()),
			_ => None,
		}
	}

	pub fn spawn_thread(&mut self, name: &str) -> Result<ThreadHandle, ThreadErr> {
		let proc = match PROCESSES.get(self.uid) {
			Some(p) => p,
			_ => return Err(ThreadErr::NoParentProcess),
		};
		let th = thread::new(*self, name);
		return match th {
			Ok(th) => {
				proc.threads.lock().insert(th);
				Ok(th)
			},
			Err(e) => Err(e),
		}
	}
}
