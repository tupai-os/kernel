// file : serial.rs
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

#[cfg(feature = "arch_target_armv7")]
use arch::arm::armv7::uart;

use core::fmt;
use spin::Mutex;

pub struct Writer {}

// TODO: Work out why static initialization doesn't work
static WRITER: Mutex<Writer> = Mutex::new(Writer {});

fn write_byte(b: u8) {
	#[cfg(feature = "arch_target_armv7")] {
		uart::write(b)
	}
}

fn read_byte() -> u8 {
	#[cfg(feature = "arch_target_armv7")] {
		uart::read()
	}
}

impl Writer {
	fn init(&mut self) {
		#[cfg(feature = "arch_target_armv7")] {
			uart::init()
		}
	}

	fn write(&mut self, c: char) {
		match c {
			'\n' => {
				write_byte(b'\n');
				write_byte(b'\r')
			}
			c => write_byte(c as u8)
		};
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
