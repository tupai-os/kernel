// file : pfa.rs
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

use arch::base::mem;

const PROC_MAX: usize = (1 << 16);
const PAGE_NUM: usize = (4 * 1024 * 1024) / mem::PAGE_SIZE_KB; // 4G of pages

const OWNER_INVALID: u32 = 0;
const OWNER_FREE: u32 = 1;
const OWNER_KERNEL: u32 = 2;

pub const ENTRY_INVALID: PageEntry = PageEntry::new(OWNER_INVALID);
pub const ENTRY_FREE_RAM: PageEntry = PageEntry::new(OWNER_FREE);

#[repr(C, packed)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct PageEntry {
	owner: u32,
}

struct PageMap {
	entries: &'static mut [PageEntry],
}

impl PageEntry {
	const fn new(owner: u32) -> PageEntry {
		PageEntry {
			owner: owner,
		}
	}
}

impl PageMap {
	fn new() -> PageMap {
		use mem::wma;
		PageMap {
			entries: wma::alloc_many::<PageEntry>(PAGE_NUM),
		}
	}

	fn clear_with(&mut self, new_entry: PageEntry) {
		for entry in self.entries.iter_mut() {
			*entry = new_entry.clone()
		}
	}

	fn set_entry(&mut self, index: usize, entry: PageEntry) -> Result<(), ()> {
		if index >= self.entries.len() {
			Err(())
		} else {
			self.entries[index] = entry;
			Ok(())
		}
	}

	fn display(&self) {
		let mut centry = ENTRY_INVALID;
		logln!("Page Map:");
		for i in 0..self.entries.len() {
			if self.entries[i] != centry || i == 0 {
				centry = self.entries[i];
				logln!("[0x{:0>18X}] owner = {}", i * mem::PAGE_SIZE, centry.owner)
			}
		}
		logln!("[0x{:0>18X}] <unmapped>", self.entries.len() * mem::PAGE_SIZE)
	}
}

use spin::Mutex;
lazy_static! {
	static ref MAP: Mutex<PageMap> = Mutex::new(PageMap::new());
}

use spin::Once;
static INIT: Once<()> = Once::new();

pub fn init() {
	INIT.call_once(|| {
		let mut map = MAP.lock();
		map.clear_with(ENTRY_INVALID);
		logok!("Initiated PFA at {:p} with {} entries", map.entries.as_ptr(), map.entries.len());
	});
}

pub enum AllocErr {
	Conflict,
	OutOfRange,
}

pub fn set_range_kb(start_kb: usize, end_kb: usize, entry: PageEntry) -> Result<(), AllocErr> {
	use util::math::kb_to_page_index;
	let start_index = kb_to_page_index(start_kb);
	let size = kb_to_page_index(end_kb - start_kb);

	let mut map = MAP.lock();
	for i in 0..size {
		match map.set_entry(start_index + i, entry) {
			Ok(()) => {},
			Err(()) => return Err(AllocErr::OutOfRange),
		}
	}

	Ok(())
}

pub fn set_range(start: usize, end: usize, entry: PageEntry) -> Result<(), AllocErr> {
	use util::math::align_up;
	set_range_kb(start >> 10, align_up(end, 10) >> 10, PageEntry::new(OWNER_KERNEL))
}

pub fn reserve_kernel() {
	use util::elf;
	set_range(elf::kernel_bounds().start, elf::kernel_bounds().end, PageEntry::new(OWNER_KERNEL));
	logok!("Reserved kernel from {:p} to {:p}", elf::kernel_bounds().start as *const (), elf::kernel_bounds().end as *const ());
}

pub fn display() {
	MAP.lock().display()
}
