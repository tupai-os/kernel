// file : mmio.rs
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

use volatile::Volatile;
pub struct Reg<T: Copy> {
	reg: Volatile<T>,
}

impl <T: Copy> Reg<T> {
	pub fn write(&mut self, val: T) {
		self.reg.write(val);
		wait(150)
	}

	pub fn read(&self) -> T {
		wait(150);
		self.reg.read()
	}
}

pub type Reg32 = Reg<u32>;

use spin::Mutex;
pub struct RegBlock<T: 'static> {
	block: Mutex<&'static mut T>,
}

use spin::MutexGuard;
impl <T> RegBlock<T> {
	pub fn new(base: usize) -> RegBlock<T> {
		RegBlock {
			block: Mutex::new(unsafe { &mut *(base as *mut _) }),
		}
	}

	pub fn lock<'a>(&self) -> MutexGuard<&'static mut T> {
		self.block.lock()
	}
}

pub fn wait(cycles: usize) {
	for i in 0..cycles {
		unsafe { asm!("") }
	}
}

pub fn write32(reg: usize, value: u32) {
	unsafe { &mut *(reg as *mut Volatile<u32>) }.write(value);
	wait(150)
}

pub fn read32(reg: usize) -> u32 {
	wait(150);
	unsafe { &mut *(reg as *mut Volatile<u32>) }.read()
}
