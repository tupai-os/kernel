// file : heap.rs
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

const BLOCK_SIZE_LOG2: usize = 5;
const BLOCK_SIZE: usize = 1 << BLOCK_SIZE_LOG2;
const BLOCK_COUNT: usize = 0x100000; // 0x20 * 0x100000 = 32M

#[repr(packed, C)]
pub struct Block {
	_unused: [u8; BLOCK_SIZE],
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
enum MapEntry { // We choose some weird bit pattersn here for error detection
	Free = 170, // 10101010
	Head = 85,  // 01010101
	Tail = 51,  // 00110011
}

pub struct Heap {
	map: usize,
	blocks: usize,
}

impl Heap {
	pub const fn empty() -> Heap {
		Heap {
			map: 0,
			blocks: 0,
		}
	}

	fn init(&mut self) {
		use mem::wma;
		self.map = wma::alloc_many::<MapEntry>(BLOCK_COUNT).as_ptr() as usize;
		self.blocks = wma::alloc_many::<Block>(BLOCK_COUNT).as_ptr() as usize;

		let map = self.get_map();

		// Free all entries
		for entry in map.iter_mut() {
			*entry = MapEntry::Free;
		}
	}

	fn get_map(&self) -> &'static mut [MapEntry] {
		use core::slice::from_raw_parts_mut;
		unsafe { from_raw_parts_mut(self.map as *mut MapEntry, BLOCK_COUNT) }
	}

	fn get_blocks(&self) -> &'static mut [Block] {
		use core::slice::from_raw_parts_mut;
		unsafe { from_raw_parts_mut(self.blocks as *mut Block, BLOCK_COUNT) }
	}

	fn index_to_ptr(&self, i: usize) -> *mut u8 {
		(self.blocks + i * BLOCK_SIZE) as *mut u8
	}

	fn ptr_to_index(&self, ptr: *mut u8) -> Option<usize> {
		let ptr = ptr as usize - self.blocks;
		if (ptr >> BLOCK_SIZE_LOG2) << BLOCK_SIZE_LOG2 != ptr {
			None
		} else {
			Some(ptr >> BLOCK_SIZE_LOG2)
		}
	}

	fn display(&self, start: usize, n: usize) {
		logln!("Heap map:");
		let map = self.get_map();
		for i in start..start + n {
			log!(""); // TODO: Work out why this line is needed to prevent an invalid op exception
			match map[i] {
				MapEntry::Free => log!("-"),
				MapEntry::Head => log!("H"),
				MapEntry::Tail => log!("T"),
				_ => log!("!"),
			}
		}
		logln!("");
	}
}

use alloc::heap::{Alloc, AllocErr, Layout};
unsafe impl<'a> Alloc for &'a Heap {
	unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
		let map = self.get_map();

		use util::math;
		let n_blocks = math::align_up(layout.size(), BLOCK_SIZE_LOG2) >> BLOCK_SIZE_LOG2;

		use core::cmp::min;
		for i in 0..map.len() {
			let mut found = true;

			if (self.index_to_ptr(i) as usize) % layout.align() != 0 {
				continue
			}

			for j in i..min(map.len(), i + n_blocks) {
				if map[j] != MapEntry::Free {
					found = false;
					break
				}
			}

			if found {
				map[i] = MapEntry::Head;
				for i in i + 1..min(map.len(), i + n_blocks) {
					map[i] = MapEntry::Tail
				}
				return Ok(self.index_to_ptr(i))
			}
		}
		Err(AllocErr::Exhausted {
			request: layout,
		})
	}

	unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
		let map = self.get_map();

		let i = self.ptr_to_index(ptr).expect("Attempted to dealloc block-unaligned pointer");

		if map[i] != MapEntry::Head {
			panic!("Attempted to dealloc unallocated pointer");
		}

		map[i] = MapEntry::Free;

		for i in i + 1..map.len() {
			match map[i] {
				MapEntry::Tail => map[i] = MapEntry::Free,
				MapEntry::Head |MapEntry::Free => break,
				_ => panic!("Error found in kernel heap"),
			}
		}
	}
}

pub fn init() {
	use HEAP;
	unsafe {
		// I wish there was nicer syntax than this. I've not found it yet.
		// Prepare for some wild casting
		let heap = &mut *((&HEAP) as *const Heap as usize as *mut Heap);
		heap.init();
	}
	logok!("Initiated heap blocks at {:p} with {} blocks", HEAP.blocks as *const (), BLOCK_COUNT);
	logok!("Initiated heap map at {:p}", HEAP.map as *const ());

	// Test everything works
	use alloc::boxed::Box;
	let x = Box::new(1337);
	if *x != 1337 {
		panic!("Heap allocation test failed");
	}
}

#[allow(dead_code)]
pub fn display(start: usize, n: usize) {
	unsafe {
		use HEAP;
		let heap = &*((&HEAP) as *const Heap);
		heap.display(start, n);
	}
}
