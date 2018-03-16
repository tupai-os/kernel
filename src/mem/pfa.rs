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
use env::env::EnvId;

bitflags! {
	pub struct Flags: u32 {
		const NONE = 0;
		const RAM  = 0b0001;
		const USED = 0b0001;
	}
}

const PROC_MAX: usize = (1 << 16);
const PAGE_NUM: usize = (4 * 1024 * 1024) / mem::PAGE_SIZE_KB; // 4G of pages

const OWNER_INVALID: EnvId = 0;
const OWNER_FREE: EnvId = 1;
const OWNER_KERNEL: EnvId = 2;

pub const ENTRY_INVALID: PageEntry = PageEntry::new(0, Flags::NONE);
pub const ENTRY_FREE_RAM: PageEntry = PageEntry::new(0, Flags::RAM);

#[repr(C, packed)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct PageEntry {
	owner: EnvId,
	flags: Flags,
}

struct PageMap {
	entries: &'static mut [PageEntry],
}

impl PageEntry {
	pub const fn new(owner: EnvId, flags: Flags) -> PageEntry {
		PageEntry {
			owner: owner,
			flags: flags,
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
		use env;

		let mut centry = ENTRY_INVALID;
		logln!("Page Map:");
		for i in 0..self.entries.len() {
			if self.entries[i] != centry || i == 0 {
				centry = self.entries[i];

				use alloc::string::ToString;
				let owner_name = match env::get(centry.owner) {
					Some(o) => o.name,
					None => "<none>".to_string(),
				};

				logln!("[0x{:0>18X}] => {:<12} owner = {} flags = 0b{:0>8b}", i * mem::PAGE_SIZE, owner_name, centry.owner, centry.flags)
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

#[derive(Debug)]
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
	set_range_kb(start >> 10, align_up(end, 10) >> 10, entry)
}

pub fn display() {
	MAP.lock().display()
}
