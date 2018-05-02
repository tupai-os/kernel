// file : irqqueue.rs
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

use util::IrqLock;
use alloc::VecDeque;
use core::cell::UnsafeCell;
use spin::Mutex;

// TODO: Is this safe?

pub struct IrqQueue<T> {
	deque: Mutex<VecDeque<T>>,
}

impl<T: Copy> IrqQueue<T> {
	pub fn new() -> IrqQueue<T> {
		IrqQueue::<T> {
			deque: Mutex::new(VecDeque::new()),
		}
	}

	pub fn write(&self, t: T) {
		let irqlock = IrqLock::new();
		self.deque.lock().push_front(t);
	}

	pub fn read(&self) -> Option<T> {
		let irqlock = IrqLock::new();
		return self.deque.lock().pop_back();
	}
}
