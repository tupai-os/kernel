// file : amd64.rs
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

global_asm!(include_str!("isr.s"));

use driver;
use arch::family::x86;
use arch::tags::multiboot;

pub fn enable_irqs() {
	unsafe { asm!("sti"); }
}

pub fn disable_irqs() {
	unsafe { asm!("cli"); }
}

pub fn halt() {
	unsafe { asm!("hlt"); }
}

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
pub extern fn kearly(tags: *const ()) {
	use kmain;

	gdt::init();
	idt::init();

	x86::init();

	multiboot::parse(tags);

	driver::init();
	let args = ["testing"];
	kmain(&args);
}
