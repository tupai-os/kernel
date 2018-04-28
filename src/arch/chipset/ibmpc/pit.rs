
// file : pit.rs
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

use llapi::intrinsic::isa::{idt, isr};
use llapi::intrinsic::family::port::out8;
use super::pic;

const IRQ: usize = 0;

const PORT_CMD: u16 = 0x43;
const PORT_DATA: u16 = 0x40;

const MAX_RATE: u32 = 1193180; // Hz

extern {
	fn _pit_handler();
}

use spin::Mutex;
static RATE: Mutex<u32> = Mutex::new(0);

pub fn init() {
	idt::set_handler(pic::REMAP_OFFSET + IRQ, _pit_handler as idt::IsrPtr);
	idt::reinstall();
	logok!("Set PIT irq handler");

	set_rate(1000);
	logok!("Set PIT rate to {} hz", *RATE.lock());

	pic::unmask(IRQ);
	logok!("Unmasked PIT interrupt");
}

pub fn set_rate(rate: u32) {
	out8(PORT_CMD, (0x3 << 1) | (0x3 << 4)); // Square-wave, lo/hi byte

	*RATE.lock() = rate;

	// Set divisor
	let div = MAX_RATE / *RATE.lock();
	out8(PORT_DATA, (div >> 0) as u8);
	out8(PORT_DATA, (div >> 8) as u8);
}

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
extern fn pit_handler(frame: *mut isr::InterruptFrame) {
	pic::ack(IRQ);
	//log!("!");
}
