// file : stack.rs
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

use llapi::cpu::irq;
use util::math::align_up;
use alloc::Vec;
use core::{
	ptr,
	ptr::Unique,
	mem::size_of,
};

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
struct StackPart {
	unused: [u8; 16],
}

pub struct Stack {
	data: Vec<StackPart>,
	pub frame: Unique<irq::StackFrame>,
}

impl StackPart {
	const fn zero() -> StackPart {
		StackPart {
			unused: [0; 16],
		}
	}
}

impl Stack {
	pub fn new(minimum_size: usize, entry: usize) -> Stack {
		let size = align_up(minimum_size, 4);
		let mut data = Vec::new();
		data.resize(size / size_of::<StackPart>(), StackPart::zero());

		// Do some funky pointer arithmetic to work out where the new stack frame goes
		let frame_ptr = data.as_ptr() as usize + size - size_of::<irq::StackFrame>();
		unsafe { ptr::write(frame_ptr as *mut _, irq::StackFrame::new(entry, data.as_ptr() as usize + size)) };

		Stack {
			data: data,
			frame: unsafe { Unique::new_unchecked(frame_ptr as *mut irq::StackFrame) },
		}
	}
}
