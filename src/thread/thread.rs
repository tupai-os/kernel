// file : thread.rs
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

use llapi::irq::InterruptFrame;

use alloc::{boxed::Box, Vec};

pub type Id = u64;
pub const ID_MAX: Id = !0;

pub struct Thread {
	id: Id,
	stack: Stack,
	frame: InterruptFrame,
}

pub type Stack = Box<Vec<u8>>;

impl Thread {
	pub fn new(id: Id, name: &str, entry: fn() -> i32, stack: Stack) -> Thread {
		let stack_ptr = stack.as_ptr() as usize;
		Thread {
			id: id,
			stack: stack,
			frame: InterruptFrame::new(entry as usize, stack_ptr),
		}
	}
}
