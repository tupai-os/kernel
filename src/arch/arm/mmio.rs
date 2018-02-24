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

// TODO: Improve this
pub fn wait(_cycles: usize) {
	use volatile::Volatile;
 	let mut i: usize = 0;
	let iv = unsafe { &mut *(&mut i as *mut usize as *mut Volatile<usize>) };
	while iv.read() < _cycles {
		let old = iv.read();
		iv.write(old + 1)
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
