
// file : com.rs
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

use arch::family::x86::port::{out8, in8};
use spin::{Mutex, Once};

static LOCK: Mutex<u32> = Mutex::new(0);
static INIT: Once<()> = Once::new();

const PORT_COM1: u16 = 0x03F8;

pub fn init() {
	INIT.call_once(|| {
		// Disable interrupts
		out8(PORT_COM1 + 1, 0x00);

		// Enable division latch access bit (set baud rate divisor)
		out8(PORT_COM1 + 3, 0x80);

		//Set divisor to 38400
		out8(PORT_COM1 + 0, 0x03);
		out8(PORT_COM1 + 1, 0x00);

		// Set 8 bits, no parity and 1 stop bits
		out8(PORT_COM1 + 3, 0x03);

		// Enable FIFO, clear with 14-byte threshold
		out8(PORT_COM1 + 2, 0xC7);

		// Reenable IRQs, RTS/DSR set
		out8(PORT_COM1 + 4, 0x0B);

		logok!("Initiated COM serial");
	});
}

pub fn write(data: u8) {
	LOCK.lock();
	while in8(PORT_COM1 + 5) & 0x20 == 0 {}
	out8(PORT_COM1, data)
}

pub fn read() -> u8 {
	LOCK.lock();
	while in8(PORT_COM1 + 5) & 0x01 == 0 {}
	in8(PORT_COM1)
}

// TODO: Use a trait to wrap tty stuff
pub fn write_char(c: char) {
	match c {
		'\n' => { write(b'\r') }
		_ => {}
	}
	write(c as u8)
}
