// file : irq.rs
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

use super::isr;

pub use self::isr::StackFrame;

pub fn enable() {
	unsafe { asm!("sti"); }
}

pub fn disable() {
	unsafe { asm!("cli"); }
}

pub fn is_enabled() -> bool {
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

pub fn await() {
	unsafe { asm!("hlt"); }
}
