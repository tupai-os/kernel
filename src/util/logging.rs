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

#[cfg(feature = "driver_vga")]
use driver::vga;

#[cfg(feature = "driver_serial")]
use driver::serial;

pub fn log_args(args: fmt::Arguments) {
	use core::fmt::Write;

	// Write to relevant driver

	#[cfg(feature = "driver_vga")] {
		vga::writer()
			.lock()
			.write_fmt(args)
			.unwrap();
	}

	#[cfg(feature = "driver_serial")] {
		serial::writer()
			.lock()
			.write_fmt(args)
			.unwrap();
	}
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
