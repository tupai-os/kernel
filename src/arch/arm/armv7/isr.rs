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
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ExceptionFrame {
	r0: u32,
	r1: u32,
	r2: u32,
	r3: u32,
	r12: u32,
	lr: u32,
	pub pc: u32,
	spsr: u32,
}

impl ExceptionFrame {
	pub fn get_instruction_ptr(&self) -> u32 {
		self.lr
	}
}

use core::fmt;
impl fmt::Display for ExceptionFrame {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		unsafe {
			write!(f,
				"\
				\tpc:   0x{:X}\n\
				\tlr:   0x{:X}\n\
				\tspsr: 0x{:X}",
				self.pc, self.lr, self.spsr
			)
		}
	}
}
