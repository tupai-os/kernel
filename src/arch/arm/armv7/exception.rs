// file : exception.rs
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

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
extern fn hwi_handler() {
	logln!("HWI occured!");
}

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
extern fn swi_handler() {
	logln!("SWI occured!");
}

extern {
	fn _exception_table_start();
	fn _exception_table_end();
}

#[no_mangle]
#[linkage = "external"]
fn relocate_exception_table() {
	let len = _exception_table_end as usize - _exception_table_start as usize;
	use util::mem;
	use core::slice;
	unsafe {
		mem::copy(
			slice::from_raw_parts(_exception_table_start as *const u8, len),
			slice::from_raw_parts_mut(0 as *mut u8, len)
		)
	}
}

pub fn init() {
	// TODO: Fix this
	//relocate_exception_table();
	logok!("Set exception handlers");
}
