// file : mod.rs
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

pub mod com;
//pub mod uart;
pub mod console;
//pub mod bcm2835;

pub struct CharIFace {
	write_char: fn(c: char)
}

pub struct ConsoleIFace {
	write_char: fn(c: char),
	clear: fn(),
	cursor_to: fn(x: u16, y: u16),
	set_color: fn(fg: u8, bg: u8),
	reset_color: fn(),
	enable_cursor: fn(),
	disable_cursor: fn(),
}

pub struct Desc {
	pub entry: fn(),
	pub char_iface: Option<CharIFace>,
	pub console_iface: Option<ConsoleIFace>,
}

use llapi::driver;

impl CharIFace {
	pub fn write_char(&self, c: char) {
		(self.write_char)(c)
	}
}

pub fn init() {
	// Initiate each boot driver
	for driver_desc in driver::ON_BOOT.iter() {
		(driver_desc.entry)();
	}
}
