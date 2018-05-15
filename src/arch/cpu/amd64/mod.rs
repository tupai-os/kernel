// file : mod.rs
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

pub mod boot;
pub mod gdt;
pub mod idt;
pub mod isr;
pub mod mem;

use super;

global_asm!(include_str!("isr.s"));

use {
	kmain,
	log,
	llapi::intrinsic::{
		family,
		chipset,
	},
	arch::tags::multiboot,
};

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
pub extern fn kearly(tags: *const ()) {
	log::init(); // Initiate early logging

	// Core architecture initiation
	gdt::init();
	idt::init();
	loginfo!("Initiated amd64 architecture");

	// Initiate other LLAPI components
	family::init();
	chipset::init();

	let boot_data = multiboot::parse(tags); // Parse tags

	kmain(&boot_data);
}

#[allow(non_camel_case_types)]
pub struct amd64 {}

impl super::Cpu for amd64 {
	type irq = Irq;

	fn name() -> &'static str { "amd64" }
}

#[allow(non_camel_case_types)]
pub struct irq {}

impl super::Irq for irq {
	fn enable() {
		unsafe { asm!("sti"); }
	}

	fn disable() {
		unsafe { asm!("cli"); }
	}

	fn await() {
		unsafe { asm!("hlt") }
	}

	fn is_enabled() -> bool {
		let val: u64;
		unsafe {
			asm!(
				"pushfq
				pop %rax"
				:"={rax}"(val)
				::: "volatile"
			);
		}
		return (val & (1 << 9)) != 0;
	}
}
