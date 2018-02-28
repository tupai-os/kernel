// file : gpio.rs
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

use arch::arm::mmio::{RegBlock, Reg32};
use arch::arm::board;

#[allow(dead_code)]
#[repr(C)]
struct GpioRegs {
	_unused0   : [Reg32; 37], // 0
	pin_pull   : Reg32, // 148
	pin_clock  : [Reg32; 2], // 152
	_unused1   : [Reg32; 2], // 156
}

lazy_static! {
	static ref GPIO: RegBlock<GpioRegs> = RegBlock::new(board::GPIO_BASE);
}

pub fn reset_pin_clock(clock: usize, pin: usize) {
	// Acquire GPIO lock
	let mut gpio = GPIO.lock();

	// Reset current pin up/down
	gpio.pin_pull.write(0);
	gpio.pin_clock[clock].write(1 << pin);
}
