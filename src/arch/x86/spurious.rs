// file : spurious.rs
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

use super::pic;
use super::isa::idt;
use super::isa::isr;

const IRQ: usize = 7;

extern {
	fn _spurious_handler();
}

pub fn init() {
	idt::set_handler(pic::REMAP_OFFSET + IRQ, _spurious_handler as idt::IsrPtr);
	idt::reinstall();
	logok!("Set spurious IRQ handler");
}

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
extern fn spurious_handler(frame: *mut isr::InterruptFrame) {
	// Do nothing
}
