// file : gdt.rs
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

const GDT_SIZE: usize = 5;

#[repr(u8)]
enum Access {
	READWRITE = 0b00000010,
	EXECUTE   = 0b00001000,
	PRESENT   = 0b10000000,
	ONE       = 0b00010000,
	KERNEL    = 0b00000000,
	USER      = 0b01100000,
}

#[repr(u8)]
enum Granularity {
	PAGE = 0b00001000,
	PM32 = 0b00000100,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct Entry {
	limit_lo: u16,
	base_lo: u16,
	base_mid: u8,
	access: u8,
	granularity: u8,
	base_hi: u8,
}

#[repr(C)]
#[repr(align(4096))]
struct Table {
	entries: [Entry; GDT_SIZE]
}

#[repr(C, packed)]
struct Ptr {
	limit: u16,
	base: u32,
}

lazy_static! {
	static ref GDT: Mutex<Table> = Mutex::new(
		Table::new_default()
	);
}

impl Entry {
	const fn null() -> Entry {
		Entry::from(0x0, 0x0, 0b0, 0b0)
	}

	const fn from(base: usize, limit: usize, access: u8, granularity: u8) -> Entry {
		Entry {
			base_lo: (base >> 0) as u16 & 0xFFFF,
			base_mid: (base >> 16) as u8 & 0xFF,
			base_hi: (base >> 24) as u8 & 0xFF,

			limit_lo: limit as u16 & 0xFFFF,

			granularity: ((limit >> 16) as u8 & 0xF) | ((granularity << 4) & 0xF0),
			access: access,
		}
	}
}

impl Table {
	fn new_default() -> Table {
		let code_access =
			Access::READWRITE as u8 |
			Access::EXECUTE as u8 |
			Access::ONE as u8 |
			Access::PRESENT as u8;

		let data_access =
			Access::READWRITE as u8 |
			Access::ONE as u8 |
			Access::PRESENT as u8;

		let kernel_code_access = code_access | Access::KERNEL as u8;
		let kernel_data_access = data_access | Access::KERNEL as u8;
		let user_code_access = code_access | Access::USER as u8;
		let user_data_access = data_access | Access::USER as u8;

		let granularity = Granularity::PAGE as u8 | Granularity::PM32 as u8;

		Table {
			entries: [
				Entry::null(),
				Entry::from(0x0, 0xFFFFF, kernel_code_access, granularity),
				Entry::from(0x0, 0xFFFFF, kernel_data_access, granularity),
				Entry::from(0x0, 0xFFFFF, user_code_access, granularity),
				Entry::from(0x0, 0xFFFFF, user_data_access, granularity),
			]
		}
	}

	fn install(&mut self) {
		let ptr = Ptr::from(self);

		unsafe {
			asm!("xchg %bx, %bx");
			asm!(
				"lgdt ($0);
				mov $$0x10, %ax;
				mov %ax, %ds;
				mov %ax, %fs;
				mov %ax, %es;
				mov %ax, %gs;
				mov %ax, %ss"
			:: "r" (&ptr) : "memory"
			)
		}
	}
}

impl Ptr {
	fn from(table: &Table) -> Ptr {
		Ptr {
			limit: (GDT_SIZE * size_of::<Entry>()) as u16 - 1,
			base: table as *const _ as u32,
		}
	}
}

pub fn init() {
	GDT.lock().install();
	//logok!("Installed GDT");
}
