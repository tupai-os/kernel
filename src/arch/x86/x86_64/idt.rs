// file : idt.rs
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

use core::mem::size_of;
use spin::Mutex;
use super::gdt;

const SIZE: usize = 256;

pub type IsrPtr = u64;

#[allow(dead_code)]
#[repr(u8)]
enum Attribute {
	TaskGate = 0b00000101,
	IntGate  = 0b00001110,
	TrapGate = 0b00001111,
	Storage  = 0b00010000,
	Dpl0     = 0b00000000,
	Dpl1     = 0b00100000,
	Dpl2     = 0b01000000,
	Dpl3     = 0b01100000,
	Present  = 0b10000000,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct Entry {
	base_lo: u16,
	selector: u16,
	zero0: u8,
	attributes: u8,
	base_mid: u16,
	base_hi: u32,
	zero1: u32,
}

#[repr(C)]
#[repr(align(4096))]
struct Table {
	entries: [Entry; SIZE]
}

#[repr(C, packed)]
struct Ptr {
	limit: u16,
	base: u64,
}

static IDT: Mutex<Table> = Mutex::new(
	Table::new_null()
);

impl Entry {
	const fn null() -> Entry {
		Entry {
			base_lo: 0,
			selector: 0,
			zero0: 0,
			attributes: 0,
			base_mid: 0,
			base_hi: 0,
			zero1: 0,
		}
	}

	fn empty() -> Entry {
		Entry::from(None)
	}

	fn from(isr: Option<u64>) -> Entry {
		let isr_addr = match isr {
			Some(i) => i,
			None => 0x0,
		};

		Entry {
			base_lo: (isr_addr >> 0) as u16 & 0xFFFF,
			base_mid: (isr_addr >> 16) as u16 & 0xFFFF,
			base_hi: (isr_addr >> 32) as u32 & 0xFFFFFFFF,

			selector: gdt::CODE_SELECTOR as u16,

			attributes:
				Attribute::IntGate as u8 |
				Attribute::Dpl0 as u8 |
				match isr {
					Some(_) => Attribute::Present as u8,
					None => 0,
				},

			zero0: 0,
			zero1: 0,
		}
	}
}

impl Table {
	const fn new_null() -> Table {
		Table {
			entries: [Entry::null(); SIZE],
		}
	}

	fn init(&mut self) {
		self.entries = [Entry::empty(); SIZE]
	}

	fn set_entry(&mut self, vec: usize, func: IsrPtr) {
		self.entries[vec] = Entry::from(Some(func));
	}

	fn install(&mut self) {
		let ptr = Ptr::from(self);

		unsafe {
			asm!(
				"lidt ($0)"
				:: "r" (&ptr) : "memory"
			)
		}
	}
}

impl Ptr {
	fn from(table: &Table) -> Ptr {
		Ptr {
			limit: (SIZE * size_of::<Entry>()) as u16 - 1,
			base: table as *const _ as u64,
		}
	}
}


pub fn init() {
	IDT.lock().init();
	IDT.lock().install();
	logok!("Installed IDT");
}

pub fn set_handler(vec: usize, func: IsrPtr) {
	IDT.lock().set_entry(vec, func);
}
