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

const SIZE: usize = 5;

pub const KERNEL_CODE_SELECTOR: usize = size_of::<Entry>() * 1;
pub const KERNEL_DATA_SELECTOR: usize = size_of::<Entry>() * 2;
pub const USER_CODE_SELECTOR: usize = size_of::<Entry>() * 3;
pub const USER_DATA_SELECTOR: usize = size_of::<Entry>() * 4;

#[allow(dead_code)]
#[repr(u8)]
enum Access {
	ReadWrite  = 0b00000010,
	Execute    = 0b00001000,
	Present    = 0b10000000,
	One        = 0b00010000,
	Kernel     = 0b00000000,
	User       = 0b01100000,
	Conforming = 0b00000100,
}

#[repr(u8)]
enum Granularity {
	Page   = 0b00001000,
	Long64 = 0b00000010,
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
	entries: [Entry; SIZE]
}

#[repr(C, packed)]
struct Ptr {
	limit: u16,
	base: u64,
}

static GDT: Mutex<Table> = Mutex::new(
	Table::new_null()
);

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
	const fn new_null() -> Table {
		Table {
			entries: [Entry::null(); SIZE],
		}
	}

	fn init(&mut self) {
		let code_access =
			Access::ReadWrite as u8 |
			Access::Execute as u8 |
			Access::One as u8 |
			Access::Conforming as u8 |
			Access::Present as u8;

		let data_access =
			Access::ReadWrite as u8 |
			Access::One as u8 |
			Access::Present as u8;

		let kernel_code_access = code_access | Access::Kernel as u8;
		let kernel_data_access = data_access | Access::Kernel as u8;
		let user_code_access = code_access | Access::User as u8;
		let user_data_access = data_access | Access::User as u8;

		let granularity =
			Granularity::Page as u8 |
			Granularity::Long64 as u8;

		self.entries = [
			Entry::null(),
			Entry::from(0x0, 0xFFFFF, kernel_code_access, granularity),
			Entry::from(0x0, 0xFFFFF, kernel_data_access, granularity),
			Entry::from(0x0, 0xFFFFF, user_code_access, granularity),
			Entry::from(0x0, 0xFFFFF, user_data_access, granularity),
		]
	}

	fn install(&mut self) {
		let ptr = Ptr::from(self);

		unsafe {
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
			limit: (SIZE * size_of::<Entry>()) as u16 - 1,
			base: table as *const _ as u64,
		}
	}
}

pub fn init() {
	GDT.lock().init();
	GDT.lock().install();
	logok!("Installed GDT");
}
