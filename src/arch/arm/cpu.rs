// file : cpu.rs
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

pub fn enable_irqs() {
	unsafe {
		asm!(
			"mrs r0, cpsr;
			bic r0, r0, #0xE0;
			msr cpsr_c, r0"
			::: "r0"
		)
	}
}

pub fn disable_irqs() {
	unsafe {
		asm!(
			"mrs r0, cpsr;
			orr r0, r0, #0xE0;
			msr cpsr_c, r0"
			::: "r0"
		)
	}
}

pub fn halt() {
	unsafe {
		asm!("wfi")
	}
}

#[allow(unused_assignments)]
pub fn get_core_number() -> u32 {
	let mut core_num: u32 = 0;
	unsafe { asm!("mrc p15, 0, $0, c0, c0, 5" : "=r"(core_num)) }
	core_num & 0x3
}
