
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

use llapi::intrinsic::{
	family::port::{out8, in8},
	chipset::pic,
	isa::{idt, isr}
};
use spin::{Mutex, Once};

const IRQ_COM1: usize = 4;
const IRQ_COM2: usize = 3;

extern {
	fn _com1_handler();
	fn _com2_handler();
}

// TODO: Why are we using a mutex here? Needs better locking
static LOCK: Mutex<u32> = Mutex::new(0);
static INIT: Once<()> = Once::new();

const PORT_COM1: u16 = 0x03F8;
const PORT_COM2: u16 = 0x02F8;
const PORT_COM3: u16 = 0x03E8;
const PORT_COM4: u16 = 0x02E8;

pub fn init() {
	INIT.call_once(|| {
		idt::set_handler(pic::REMAP_OFFSET + IRQ_COM1, _com1_handler as idt::IsrPtr);
		idt::set_handler(pic::REMAP_OFFSET + IRQ_COM2, _com2_handler as idt::IsrPtr);
		idt::reinstall();
		logok!("Set COM1 interrupt handler");
		logok!("Set COM2 interrupt handler");

		// Disable COM1 interrupts
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
		out8(PORT_COM1 + 1, 0x01);

		pic::unmask(IRQ_COM1);
		logok!("Unmasked COM1 interrupt");
		pic::unmask(IRQ_COM2);
		logok!("Unmasked COM2 interrupt");

		loginfo!("Initiated COM serial");
	});
}

// TODO: These interrupt handlers contain potential deadlocks!

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
extern fn com1_handler(frame: *mut isr::StackFrame) -> *mut isr::StackFrame {
	logln!("COM1 INPUT: {}", read() as char);
	pic::eoi(IRQ_COM1);
	return frame;
}

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
extern fn com2_handler(frame: *mut isr::StackFrame) -> *mut isr::StackFrame {
	logln!("COM2 INPUT");
	pic::eoi(IRQ_COM2);
	return frame;
}

pub fn write(data: u8) {
	LOCK.lock();
	while in8(PORT_COM1 + 5) & 0x20 == 0 {}
	out8(PORT_COM1, data);
}

pub fn read() -> u8 {
	LOCK.lock();
	while in8(PORT_COM1 + 5) & 0x01 == 0 {}
	return in8(PORT_COM1);
}

// TODO: Use a trait to wrap tty stuff
pub fn write_char(c: char) {
	if c == '\n' {
		write(b'\r');
	}
	write(c as u8);
}
