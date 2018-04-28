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

use core::fmt;

pub trait Frame {
	fn get_instruction_ptr(&self) -> u32;
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ExceptionFrame {
	ebp: u32,
	edi: u32,
	esi: u32,
	edx: u32,
	ecx: u32,
	ebx: u32,
	eax: u32,
	kind: u32,
	error: u32,
	eip: u32,
	cs: u32,
	eflags: u32,
	esp: u32,
	ss: u32,
}

#[allow(dead_code)]
#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct InterruptFrame {
	ebp: u32,
	edi: u32,
	esi: u32,
	edx: u32,
	ecx: u32,
	ebx: u32,
	eax: u32,
	eip: u32,
	cs: u32,
	eflags: u32,
	esp: u32,
	ss: u32,
}

impl InterruptFrame {
	pub fn new(entry: usize, stack: usize) -> InterruptFrame {
		InterruptFrame {
			ebp: stack as u32,
			eip: entry as u32,
			esp: stack as u32,
			..Default::default()
		}
	}
}

impl Frame for ExceptionFrame {
	fn get_instruction_ptr(&self) -> u32 {
		self.eip
	}
}
impl Frame for InterruptFrame {
	fn get_instruction_ptr(&self) -> u32 {
		self.eip
	}
}

impl fmt::Display for ExceptionFrame {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f,
			"\
			\tkind:  {}\n\
			\terror: 0x{:X}\n\
			\teip:   0x{:X}\n\
			\tesp:   0x{:X}\n\
			\tcs:    0x{:X}\n\
			\tss:    0x{:X}\n",
			self.kind, self.error, self.eip, self.esp, self.cs, self.ss
		)
	}
}

impl fmt::Display for InterruptFrame {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		unsafe {
			writeln!(f,
				"\
				\teip:   0x{:X}\n\
				\tesp:   0x{:X}\n\
				\tcs:    0x{:X}\n\
				\tss:    0x{:X}\n",
				self.eip, self.esp, self.cs, self.ss
			)
		}
	}
}
