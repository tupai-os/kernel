
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
		isa::{
			idt,
			isr,
		},
		family::port::{
			out8,
			in8,
		},
	},
	util::IrqLock,
	spin::Mutex,
	alloc::VecDeque,
};

use super::pic;

const IRQ: usize = 1;

extern {
	fn _kbd_handler();
}

const PORT_CMD: u16 = 0x60;
const PORT_STATUS: u16 = 0x64;
const PORT_DATA: u16 = 0x60;

const KEY_CODE: u8 = !(1 << 7);
const KEY_PRESSED: u8 = 1 << 7;

const KEY_LSHIFT: u8 = 0x2A;
const KEY_LCTRL: u8 = 0x1D;

// TODO: Get rid of this hack, write a proper keyboard driver
lazy_static! {
	pub static ref CHAR_BUFFER: Mutex<VecDeque<char>> = Mutex::new(VecDeque::new());
	pub static ref MODS: Mutex<[bool; 3]> = Mutex::new([false, false, false]);
}

const MOD_SHIFT: usize = 0;
const MOD_CTRL: usize = 1;
const MOD_SUPER: usize = 2;

const SCANCODES_US: [char; 128] = [
	'!', '\x1B', '1', '2', '3', '4', '5', '6', '7', '8',	// 9
	'9', '0', '-', '=',
	'\x08', // Backspace
	'\t', // Tab
	'q', 'w', 'e', 'r',	// 19
	't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n', // Enter key
	'!', // 29 - Left control
	'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', // 39
	'\'', '`',
	'!', // Left shift
	'\\', 'z', 'x', 'c', 'v', 'b', 'n', // 49
	'm', ',', '.', '/',
	'!', // Right shift
	'*',
	'!',  // Alt
	' ', // Space bar
	'!', // Caps lock
	'!', // 59 - F1 key ... >
	'!', '!',   '!',   '!',   '!',   '!',   '!',   '!',
	'!', // < ... F10
	'!', // 69 - Num lock
	'!', // Scroll Lock
	'!', // Home key
	'!', // Up Arrow
	'!', // Page Up
	'-',
	'!', // Left Arrow
	'!',
	'!', // Right Arrow
	'+',
	'!', // 79 - End key
	'!', // Down Arrow
	'!', // Page Down
	'!', // Insert Key
	'!', // Delete Key
	'!', '!', '!',
	'!', // F11 Key
	'!', // F12 Key
	// All other keys are undefined
	'!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!',
	'!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!',
	'!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!'
];

pub fn init() {
	idt::set_handler(pic::REMAP_OFFSET + IRQ, _kbd_handler as idt::IsrPtr);
	idt::reinstall();
	logok!("Set keyboard interrupt handler");

	pic::unmask(IRQ);
	logok!("Unmasked keyboard interrupt");

	loginfo!("Initiated keyboard");
}

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
extern fn kbd_handler(frame: *mut isr::InterruptFrame) -> *mut isr::InterruptFrame {
	let mut scancodes: [u8; 6];
	let mut index = 0;

	while in8(PORT_STATUS) & 1 != 0 {
		let sc = in8(PORT_DATA);
		if sc & KEY_PRESSED == 0 {
			let c = SCANCODES_US[sc as usize];
			//logln!("0x{:X}: {} PRESSED", sc, c);

			match sc & KEY_CODE {
				KEY_LSHIFT => { MODS.lock()[MOD_SHIFT] = true; },
				KEY_LCTRL => { MODS.lock()[MOD_CTRL] = true; },
				_ => {},
			}

			let irqlock = IrqLock::new();
			CHAR_BUFFER.lock().push_back(c);
			// Drop irqlock
		} else {
			//logln!("0x{:X}: {} RELEASED", sc, SCANCODES_US[sc as usize - 128]);

			match sc & KEY_CODE {
				KEY_LSHIFT => { MODS.lock()[MOD_SHIFT] = false; },
				KEY_LCTRL => { MODS.lock()[MOD_CTRL] = false; },
				_ => {},
			}
		}

		//logln!("Modifier keys: {:?}", *MODS.lock());
	}

	pic::eoi(IRQ);
	return frame;
}
