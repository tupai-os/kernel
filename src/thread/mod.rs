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

pub struct Thread {
	pub name: String,
}

lazy_static! {
	static ref THREADS: Tracker<Thread> = Tracker::new();
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ThreadHandle {
	uid: Uid,
}

#[derive(Debug)]
pub enum ThreadErr {}

impl Thread {
	pub fn new(name: &str) -> Result<ThreadHandle, ThreadErr> {
		return Ok(ThreadHandle {
			uid: THREADS.emplace(Thread {
				name: String::from(name),
			}).0
		});
	}
}

impl ThreadHandle {
	pub fn uid(&self) -> Uid {
		self.uid
	}

	pub fn name(&self) -> Option<String> {
		match THREADS.get(self.uid) {
			Some(t) => Some(t.name.clone()),
			_ => None,
		}
	}
}
