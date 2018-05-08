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
mod preempt;
mod stack;

// Reexports
pub use self::handle::ThreadHandle as ThreadHandle;
pub use self::preempt::preempt as preempt;
pub use self::stack::Stack as Stack;

use util::{
	uid::Tracker,
	IrqLock,
};
use process::ProcessHandle;
use alloc::string::String;

pub struct Thread {
	name: String,
	proc: ProcessHandle,
	stack: Stack,
}

lazy_static! {
	static ref THREADS: Tracker<IrqLock<Thread>> = Tracker::new();
}

#[derive(Debug)]
pub enum ThreadErr {
	NoParentProcess,
}

pub fn new(proc: ProcessHandle, name: &str, entry: fn()) -> Result<ThreadHandle, ThreadErr> {
	if !proc.valid() {
		return Err(ThreadErr::NoParentProcess);
	}
	return Ok(ThreadHandle::from_uid(
		THREADS.emplace(IrqLock::new(Thread {
			name: String::from(name),
			proc: proc,
			// TODO: Specify this better?
			stack: Stack::new(4096, entry as *const () as usize),
		})).0
	));
}
