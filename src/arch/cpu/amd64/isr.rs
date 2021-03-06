// file : isr.rs
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

use super::gdt;
use core::fmt;

#[allow(dead_code)]
#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct StackFrame {
	rbp: u64,
	r15: u64,
	r14: u64,
	r13: u64,
	r12: u64,
	r11: u64,
	r10: u64,
	r9: u64,
	r8: u64,
	rdi: u64,
	rsi: u64,
	rdx: u64,
	rcx: u64,
	rbx: u64,
	rax: u64,
	kind: u64,
	error: u64,
	rip: u64,
	cs: u64,
	rflags: u64,
	rsp: u64,
	ss: u64,
}

impl StackFrame {
	pub fn new(entry: usize, stack: usize) -> StackFrame {
		StackFrame {
			rbp: stack as u64,
			rip: entry as u64,
			cs: gdt::KERNEL_CODE_SELECTOR as u64,
			rflags: 0x200, // Interrupts enabled
			rsp: stack as u64,
			ss: gdt::KERNEL_DATA_SELECTOR as u64,
			..Default::default()
		}
	}

	fn get_instruction_ptr(&self) -> u64 { self.rip }
}

impl fmt::Display for StackFrame {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		unsafe {
			write!(f,
				"\
				\trip:   0x{:X}\n\
				\trsp:   0x{:X}\n\
				\tkind:  0x{:X}\n\
				\terror: 0x{:X}\n\
				\tcs:    0x{:X}\n\
				\tss:    0x{:X}",
				self.rip, self.rsp, self.kind, self.error, self.cs, self.ss
			)
		}
	}
}
