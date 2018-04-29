
// file : kbd.rs
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

use {
	llapi::intrinsic::{
		isa::{idt, isr},
		family::port::{
			out8,
			in8,
		},
	},
	spin::Mutex,
};

use super::pic;

const IRQ: usize = 1;

extern {
	fn _kbd_handler();
}

const PORT_CMD: u16 = 0x60;
const PORT_STATUS: u16 = 0x64;
const PORT_DATA: u16 = 0x60;

// TODO: Get rid of this hack, write a proper keyboard driver
lazy_static! {
	static ref shift_down: Mutex<bool> = Mutex::new(false);
}

const SCANCODES_UK: [char; 128] = [
	'!', '\x1B', '1', '2', '3', '4', '5', '6', '7', '8',	// 9
	'9', '0', '-', '=', '\x08', // Backspace
	'\t', // Tab
	'q', 'w', 'e', 'r',	// 19
	't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n', // Enter key
	'!', // 29 - Control
	'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', // 39
	'\'', '`',
	'!',   // Left shift
	'\\', 'z', 'x', 'c', 'v', 'b', 'n', // 49
	'm', ',', '.', '/',
	'!',   // Right shift
	'*',
	'!',   // Alt
	' ', // Space bar
	'!',   // Caps lock
	'!',   // 59 - F1 key ... >
	'!',   '!',   '!',   '!',   '!',   '!',   '!',   '!',
	'!',   // < ... F10
	'!',   // 69 - Num lock
	'!',   // Scroll Lock
	'!',   // Home key
	'!',   // Up Arrow
	'!',   // Page Up
	'-',
	'!',   // Left Arrow
	'!',
	'!',   // Right Arrow
	'+',
	'!',   // 79 - End key
	'!',   // Down Arrow
	'!',   // Page Down
	'!',   // Insert Key
	'!',   // Delete Key
	'!', '!', '!',
	'!',   // F11 Key
	'!',   // F12 Key
	// All other keys are undefined
	'!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!',
	'!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!',
	'!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!'
];

pub fn init() {
	idt::set_handler(pic::REMAP_OFFSET + IRQ, _kbd_handler as idt::IsrPtr);
	idt::reinstall();
	logok!("Set keyboard irq handler");

	pic::unmask(IRQ);
	logok!("Unmasked keyboard interrupt");
}

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
extern fn kbd_handler(frame: *mut isr::InterruptFrame) -> *mut isr::InterruptFrame {
	let mut scancodes: [u8; 6];
	let mut index = 0;

	while in8(PORT_STATUS) & 1 != 0 {
		match in8(PORT_DATA) {
			sc if sc & (1 << 7) != 0 => {
				logln!("0x{:X}: {} RELEASED", sc, SCANCODES_UK[sc as usize - 128]);
			}
			sc if sc & (1 << 7) == 0 => {
				logln!("0x{:X}: {} PRESSED", sc, SCANCODES_UK[sc as usize]);
			}
			_ => {}
		}
	}

	pic::eoi(IRQ);
	return frame;
}
