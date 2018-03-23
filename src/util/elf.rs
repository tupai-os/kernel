// file : elf.rs
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

extern {
	static _kernel_start: u8;
	static _kernel_end: u8;

	static _wma_start: u8;
	static _wma_end: u8;
}

pub struct Bounds {
	pub start: usize,
	pub end: usize,
}

fn location_of(loc: &u8) -> usize {
	loc as *const u8 as usize
}

pub fn kernel_bounds() -> Bounds {
	Bounds {
		start: location_of(unsafe { &_kernel_start }),
		end: location_of(unsafe { &_kernel_end }),
	}
}

pub fn wma_bounds() -> Bounds {
	Bounds {
		start: location_of(unsafe { &_wma_start }),
		end: location_of(unsafe { &_wma_end }),
	}
}
