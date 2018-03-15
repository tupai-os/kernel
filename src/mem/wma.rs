// file : wma.rs
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

use util::elf;

use spin::Mutex;
lazy_static! {
	static ref END: Mutex<usize> = Mutex::new(elf::wma_bounds().start);
}

pub fn alloc_one<T>() -> &'static mut T {
	use core::mem;
	let cend = *END.lock();
	*END.lock() = cend + mem::size_of::<T>(); // Increment watermark
	unsafe { &mut *(cend as *mut T) }
}

pub fn alloc_many<T>(n: usize) -> &'static mut [T] {
	use core::{mem, slice};
	let cend = *END.lock();
	*END.lock() = cend + mem::size_of::<T>() * n; // Increment watermark
	unsafe { slice::from_raw_parts_mut(cend as *mut _, n) }
}

use spin::Once;
static INIT: Once<()> = Once::new();

pub fn init() {
	INIT.call_once(|| {
		logok!("Initiated WMA from 0x{:X} to 0x{:X}",
			elf::kernel_bounds().start,
			elf::kernel_bounds().end
		);
	});
}
