// file : preempt.rs
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

use super::{THREADS, Thread, ThreadHandle};
use llapi::irq;
use spin::Mutex;
use alloc::{
	arc::{Arc, Weak},
	VecDeque,
};
use core::ptr::Unique;

lazy_static! {
	// TODO: Use IRQ lock here (for both internal and external mutex)
	static ref TASK_QUEUE: Mutex<VecDeque<Weak<Mutex<Thread>>>> = Mutex::new(VecDeque::new());
	static ref CURRENT_TASK: Mutex<Weak<Mutex<Thread>>> = Mutex::new(Weak::default());
}

pub fn preempt(frame: *mut irq::StackFrame) -> *mut irq::StackFrame {
	if let Some(ct) = CURRENT_TASK.lock().upgrade() {
		ct.lock().stack.frame = unsafe { Unique::new_unchecked(frame) };
		TASK_QUEUE.lock().push_front(Arc::downgrade(&ct));
	}

	*CURRENT_TASK.lock() = TASK_QUEUE.lock().pop_back().unwrap_or_else(||{
		panic!("No more tasks, scheduler stalled");
	});

	let frame = match CURRENT_TASK.lock().upgrade() {
		Some(ct) => ct.lock().stack.frame.as_ptr(),
		_ => panic!("Encountered invalid thread"),
	};

	return frame;
}

pub fn schedule(th: ThreadHandle) -> bool {
	return match THREADS.get(th.uid()) {
		Some(t) => {
			TASK_QUEUE.lock().push_front(Arc::downgrade(&t));
			true
		},
		_ => false,
	}
}
