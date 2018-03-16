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
	let alloc_size = mem::size_of::<T>();

	if cend + alloc_size > elf::wma_bounds().end {
		panic!("Attempted to allocate past bounds of WMA")
	}

	*END.lock() = cend + alloc_size; // Increment watermark
	unsafe { &mut *(cend as *mut T) }
}

pub fn alloc_many<T>(n: usize) -> &'static mut [T] {
	use core::{mem, slice};
	let cend = *END.lock();
	let alloc_size = mem::size_of::<T>() * n;

	if cend + alloc_size > elf::wma_bounds().end {
		panic!("Attempted to allocate past bounds of WMA")
	}

	*END.lock() = cend + alloc_size; // Increment watermark
	unsafe { slice::from_raw_parts_mut(cend as *mut _, n) }
}

use spin::Once;
static INIT: Once<()> = Once::new();

pub fn init() {
	INIT.call_once(|| {
		logok!("Initiated WMA from {:p} to {:p}",
			elf::wma_bounds().start as *const (),
			elf::wma_bounds().end as *const ()
		);
	});
}
