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

pub mod mmio;
pub mod cpu;
pub mod exception;

pub mod boards;
pub use arch::arm::boards::board as board;

#[cfg(feature = "arch_target_armv7")] pub mod armv7;
#[cfg(feature = "arch_target_armv7")] pub use arch::arm::armv7 as target;

#[cfg(feature = "arch_target_armv8")] pub mod armv8;
#[cfg(feature = "arch_target_armv8")] pub use arch::arm::armv8 as target;

#[cfg(feature = "driver_serial")]
use driver::serial;

pub fn env_setup() {
	// Only continue if we're the primary core
	if cpu::get_core_number() != 0 {
		loop { cpu::halt() }
	}

	// Setup the serial driver first - we need it to display logs!
	#[cfg(feature = "driver_serial")] {
		serial::init();
	}

	target::env_setup();

	exception::init();
}

#[no_mangle]
#[linkage = "external"]
extern fn __aeabi_unwind_cpp_pr0() {}

#[no_mangle]
#[linkage = "external"]
extern fn __aeabi_unwind_cpp_pr1() {}
