// file : console.rs
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

use llapi::{
	chipset::intrinsic::regions::VGA_TEXTMODE_RAM,
	family::intrinsic::port::out8,
};
use driver::{Desc, CharIFace};
use volatile::Volatile;
use spin::{Mutex, Once};
use core::ptr::Unique;

pub const COLS: usize = 80;
pub const ROWS: usize = 25;
pub const TAB_WIDTH: usize = 4;

const PORT_CMD: u16 = 0x03D4;
const PORT_DATA: u16 = 0x03D5;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Color {
	Black      = 0,
	Blue       = 1,
	Green      = 2,
	Cyan       = 3,
	Red        = 4,
	Magenta    = 5,
	Brown      = 6,
	LightGray  = 7,
	DarkGray   = 8,
	LightBlue  = 9,
	LightGreen = 10,
	LightCyan  = 11,
	LightRed   = 12,
	Pink       = 13,
	Yellow     = 14,
	White      = 15,
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
struct Entry {
	c: u8,
	fmt: u8,
}

pub struct Writer {
	cursor: usize,
	fg_color: Color,
	bg_color: Color,
	buffer: Unique<[Volatile<Entry>; COLS * ROWS]>,
}

// TODO: Work out why static initialization doesn't work
// TODO: Use lazy_static here
static WRITER: Mutex<Writer> = Mutex::new(Writer {
	cursor: 0,
	fg_color: Color::White,
	bg_color: Color::Black,
	buffer: unsafe { Unique::new_unchecked(0 as *mut _) },
});

extern {
	fn _vga_boot_cursor() -> usize;
}

// Driver interface

pub const DESC: Desc = Desc {
	entry: init,
	char_iface: Some(CharIFace{
		write_char: write_char,
	}),
	console_iface: None,
};

fn colors_to_fmt(fg: Color, bg: Color) -> u8 {
	((bg as u8) << 4) | fg as u8
}

fn move_cursor(pos: u16) {
	// Set lo byte
	out8(PORT_CMD, 0x0F);
	out8(PORT_DATA, pos as u8);
	// Set hi byte
	out8(PORT_CMD, 0x0E);
	out8(PORT_DATA, (pos >> 8) as u8);
}

impl Entry {
	fn empty(fg: Color, bg: Color) -> Entry {
		Entry {
			c: b' ',
			fmt: colors_to_fmt(fg, bg),
		}
	}
}

impl Writer {
	fn init(&mut self) {
		self.cursor = unsafe { _vga_boot_cursor() };
		self.fg_color = Color::White;
		self.bg_color = Color::Black;
		self.buffer = unsafe { Unique::new_unchecked(VGA_TEXTMODE_RAM as *mut _) };

		enable_cursor();
		move_cursor(self.cursor as u16);
	}

	fn write(&mut self, c: u8) {
		match c {
			b'\n' => self.cursor += COLS - (self.cursor % COLS),
			b'\t' => self.cursor += TAB_WIDTH - (self.cursor % TAB_WIDTH),
			b'\x08' => {
				self.cursor -= 1;
				let cursor = self.cursor;
				let fmt = colors_to_fmt(self.fg_color, self.bg_color);
				self.buffer()[cursor].write(Entry {
					c: b' ',
					fmt: fmt,
				});
			},
			c => {
				let cursor = self.cursor;
				let fmt = colors_to_fmt(self.fg_color, self.bg_color);
				self.buffer()[cursor].write(Entry {
					c: c,
					fmt: fmt,
				});
				self.cursor += 1;
			}
		};

		while self.cursor >= COLS * ROWS {
			self.scroll(1);
			self.cursor -= COLS
		}

		move_cursor(self.cursor as u16);
	}

	fn scroll(&mut self, lines: usize) {
		let chars = lines * COLS;
		for i in 0..COLS * (ROWS - lines) {
			let old = self.buffer()[COLS + i].read();
			self.buffer()[i].write(old);
		}
		for i in 0..chars {
			let blank = Entry::empty(self.fg_color, self.bg_color);
			self.buffer()[COLS * ROWS - (i + 1)].write(blank);
		}
	}

	fn buffer(&mut self) -> &mut [Volatile<Entry>; COLS * ROWS] {
		unsafe { self.buffer.as_mut() }
	}
}

static INIT: Once<()> = Once::new();

pub fn init() {
	INIT.call_once(|| {
		WRITER.lock().init();
		logok!("Initiated console driver");
	});
}

pub fn write_char(c: char) {
	WRITER.lock().write(c as u8)
}

pub fn enable_cursor() {
	out8(PORT_CMD, 0x0A);
	out8(PORT_DATA, 0x00);
}

pub fn disable_cursor() {
	out8(PORT_CMD, 0x0A);
	out8(PORT_DATA, 0x3F);
}
