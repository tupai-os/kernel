// file : handle.rs
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

use super::PROCESSES;
use util::uid::Uid;
use thread;
use thread::{ThreadHandle, ThreadErr};
use alloc::{
	string::String,
	Vec,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessHandle {
	uid: Uid,
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
			Some(p) => Some(p.lock().name.clone()),
			_ => None,
		}
	}

	pub fn threads(&self) -> Option<Vec<ThreadHandle>> {
		match PROCESSES.get(self.uid) {
			Some(p) => Some(p.lock().threads.iter().cloned().collect()),
			_ => None,
		}
	}

	pub fn valid(&self) -> bool {
		return match PROCESSES.get(self.uid) {
			Some(_) => true,
			_ => false,
		};
	}

	pub fn spawn_thread(&self, name: &str, entry: fn()) -> Result<ThreadHandle, ThreadErr> {
		let proc = match PROCESSES.get(self.uid) {
			Some(p) => p,
			_ => return Err(ThreadErr::NoParentProcess),
		};
		let th = thread::new(*self, name, entry);
		return match th {
			Ok(th) => {
				proc.lock().threads.insert(th);
				th.schedule();
				Ok(th)
			},
			Err(e) => Err(e),
		}
	}
}
