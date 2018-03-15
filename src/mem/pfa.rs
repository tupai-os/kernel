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

const OWNER_INVALID: u16 = 0;
const OWNER_FREE: u16 = 1;

pub const RAM_INVALID: PageEntry = PageEntry::invalid();
pub const RAM_FREE: PageEntry = PageEntry::free();

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PageEntry {
	owner: u16,
	attr: u16,
}

struct PageMap {
	entries: &'static mut [PageEntry],
}

impl PageEntry {
	const fn invalid() -> PageEntry {
		PageEntry {
			owner: OWNER_INVALID,
			attr: 0b0,
		}
	}

	const fn free() -> PageEntry {
		PageEntry {
			owner: OWNER_FREE,
			attr: 0b0,
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
}

use spin::Mutex;
lazy_static! {
	static ref MAP: Mutex<PageMap> = Mutex::new(PageMap::new());
}

use spin::Once;
static INIT: Once<()> = Once::new();

pub fn init() {
	INIT.call_once(|| {
		MAP.lock().clear_with(PageEntry::invalid());

		logok!("Initiated PFA with {} entries", MAP.lock().entries.len());
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

	logln!("Len = {}", size);

	let mut map = MAP.lock();
	for i in 0..size {
		match map.set_entry(start_index + i, entry) {
			Ok(()) => {},
			Err(()) => return Err(AllocErr::OutOfRange),
		}
	}

	Ok(())
}
