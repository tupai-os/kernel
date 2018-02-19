// file : vga.rs
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

use core::ptr::Unique;
use core::fmt;
use volatile::Volatile;
use spin::Mutex;

pub const VIRT_OFFSET: usize = 0xFFFFFFFF80000000;
pub const VBUFFER: usize = 0xB8000;

pub const COLS: usize = 80;
pub const ROWS: usize = 25;
pub const TAB_WIDTH: usize = 4;

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
#[repr(packed)]
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
static WRITER: Mutex<Writer> = Mutex::new(Writer {
	cursor: 0,
	fg_color: Color::White,
	bg_color: Color::Black,
	buffer: unsafe { Unique::new_unchecked(0 as *mut _) },
});

extern {
	fn _vga_boot_cursor() -> usize;
}

fn colors_to_fmt(fg: Color, bg: Color) -> u8 {
	((bg as u8) << 4) | fg as u8
}

impl Writer {
	fn init(&mut self) {
		self.cursor = unsafe { _vga_boot_cursor() };
		self.fg_color = Color::White;
		self.bg_color = Color::Black;
		self.buffer = unsafe { Unique::new_unchecked((VIRT_OFFSET + VBUFFER) as *mut _) };
	}

	fn write(&mut self, c: char) {
		match c {
			'\n' => self.cursor += COLS - (self.cursor % COLS),
			'\t' => self.cursor += TAB_WIDTH - (self.cursor % TAB_WIDTH),
			c => {
				let cursor = self.cursor;
				let fmt = colors_to_fmt(self.fg_color, self.bg_color);
				self.buffer()[cursor].write(Entry {
					c: c as u8,
					fmt: fmt,
				});
				self.cursor += 1;
			}
		};

		if self.cursor >= COLS * ROWS {
			self.cursor = 0 // TODO: Add proper scrolling
		}
	}

	fn buffer(&mut self) -> &mut [Volatile<Entry>; COLS * ROWS] {
		unsafe { self.buffer.as_mut() }
	}
}

impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		for c in s.chars() {
			self.write(c)
		}
		Ok(())
	}
}

pub fn init() {
	WRITER.lock().init();
}

pub fn writer() -> &'static Mutex<Writer> {
	&WRITER
}
