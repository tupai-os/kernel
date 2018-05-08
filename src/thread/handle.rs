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

use super::THREADS;
use super::preempt;
use util::uid::Uid;
use alloc::string::String;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadHandle {
	uid: Uid,
}

impl ThreadHandle {
	pub const fn from_uid(uid: Uid) -> ThreadHandle {
		ThreadHandle {
			uid: uid,
		}
	}

	pub fn uid(&self) -> Uid {
		self.uid
	}

	pub fn name(&self) -> Option<String> {
		match THREADS.get(self.uid) {
			Some(t) => Some(t.lock().name.clone()),
			_ => None,
		}
	}

	pub fn schedule(&self) {
		preempt::schedule(*self);
	}
}
