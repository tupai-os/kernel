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

#[allow(dead_code)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExceptionFrame {
	ebp: u32,
	edi: u32,
	esi: u32,
	edx: u32,
	ecx: u32,
	ebx: u32,
	eax: u32,
	error: u32,
	eip: u32,
	cs: u32,
	eflags: u32,
	esp: u32,
	ss: u32,
}

impl ExceptionFrame {
	pub fn get_instruction_ptr(&self) -> u32 {
		self.eip
	}
}

use core::fmt;
impl fmt::Display for ExceptionFrame {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f,
			"\
			\teip: 0x{:x}\n\
			\tesp: 0x{:x}\n\
			\tcs:  0x{:x}\n\
			\tss:  0x{:x}\n",
			self.eip, self.esp, self.cs, self.ss
		)
	}
}
