// file : a32.rs
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
pub mod isr;
pub mod mem;

global_asm!(include_str!("exception.s"));

use arch::family::arm;
use arch::tags::atags;

pub fn enable_irqs() {
	unsafe {
		asm!(
			"mrs r0, cpsr;
			bic r0, r0, #0xE0;
			msr cpsr_c, r0"
			::: "r0"
		);
	}
}

pub fn disable_irqs() {
	unsafe {
		asm!(
			"mrs r0, cpsr;
			orr r0, r0, #0xE0;
			msr cpsr_c, r0"
			::: "r0"
		);
	}
}

pub fn halt() {
	unsafe {
		asm!("wfi");
	}
}

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
pub extern fn kearly(tags: *const ()) {
	use kmain;

	arm::init();

	atags::parse(tags);

	let args = ["testing"];
	kmain(&args);
}
