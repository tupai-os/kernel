// file : logging.rs
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

#![allow(unused_macros)]

use core::fmt;

#[cfg(feature = "driver_ttyout_com")]            use driver::serial::com    as ttyout;
#[cfg(feature = "driver_ttyout_vgaconsole")]     use driver::video::vga     as ttyout;
#[cfg(feature = "driver_ttyout_uart")]           use driver::serial::uart   as ttyout;
#[cfg(feature = "driver_ttyout_bcm283xconsole")] use driver::video::bcm283x as ttyout;

struct Writer {}

impl Writer {
	fn write(&self, c: char) {
		ttyout::write_char(c);
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

use spin::Mutex;
static WRITER: Mutex<Writer> = Mutex::new(Writer {});

pub fn log_args(args: fmt::Arguments) {
	use core::fmt::Write;
	WRITER.lock().write_fmt(args).unwrap();
}

macro_rules! log {
	($($arg:tt)*) => (
		$crate::util::logging::log_args(format_args!($($arg)*))
	);
}

macro_rules! logln {
	($fmt:expr) => (log!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)*) => (log!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! loginfo {
	($fmt:expr) => ({
		log!("[INFO] ");
		log!(concat!($fmt, "\n"))
	});
	($fmt:expr, $($arg:tt)*) => ({
		log!("[INFO] ");
		log!(concat!($fmt, "\n"), $($arg)*)
	});
}

macro_rules! logok {
	($fmt:expr) => ({
		log!("[ OK ] ");
		log!(concat!($fmt, "\n"))
	});
	($fmt:expr, $($arg:tt)*) => ({
		log!("[ OK ] ");
		log!(concat!($fmt, "\n"), $($arg)*)
	});
}

macro_rules! logfail {
	($fmt:expr) => ({
		log!("[FAIL] ");
		log!(concat!($fmt, "\n"))
	});
	($fmt:expr, $($arg:tt)*) => ({
		log!("[FAIL] ");
		log!(concat!($fmt, "\n"), $($arg)*)
	});
}
