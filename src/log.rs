// file : log.rs
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

use {
	core::fmt,
	spin::{
		Mutex,
		Once,
	},
};

use llapi::driver::LOG;

struct Writer {}

impl Writer {
	fn write(&self, c: char) {
		match LOG.char_iface {
			Some(ref cif) => cif.write_char(c),
			None => panic!("Invalid logging interface"),
		}
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

static WRITER: Mutex<Writer> = Mutex::new(Writer {});

pub fn log_args(args: fmt::Arguments) {
	use core::fmt::Write;
	WRITER.lock().write_fmt(args).unwrap();
}

pub unsafe fn force_unlock() {
	WRITER.force_unlock()
}

macro_rules! log {
	($($arg:tt)*) => (
		$crate::log::log_args(format_args!($($arg)*))
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

static INIT: Once<()> = Once::new();

pub fn init() {
	INIT.call_once(|| {
		logok!("Logging initiated");
	});
}
