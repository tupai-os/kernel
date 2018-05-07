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

use alloc::string::String;
use util::uid::{Uid, Tracker};

pub struct Process {
	pub name: String,
}

lazy_static! {
	static ref PROCESSES: Tracker<Process> = Tracker::new();
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ProcessHandle {
	uid: Uid,
}

#[derive(Debug)]
pub enum ProcessErr {}

impl Process {
	pub fn new(name: &str) -> Result<ProcessHandle, ProcessErr> {
		return Ok(ProcessHandle {
			uid: PROCESSES.emplace(Process {
				name: String::from(name),
			}).0
		});
	}
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

	pub fn uid(&self) -> Uid {
		self.uid
	}

	pub fn name(&self) -> Option<String> {
		match PROCESSES.get(self.uid) {
			Some(t) => Some(t.name.clone()),
			_ => None,
		}
	}
}
