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

pub mod port;
pub mod pic;
pub mod exception;
pub mod cpu;

#[cfg(feature = "arch_target_i386")] pub mod i386;
#[cfg(feature = "arch_target_i386")] pub use arch::x86::i386 as target;

#[cfg(feature = "arch_target_x86_64")] pub mod x86_64;
#[cfg(feature = "arch_target_x86_64")] pub use arch::x86::x86_64 as target;

#[cfg(feature = "driver_vga")]
use driver::vga;

pub fn env_setup() {
	// Setup the VGA driver first - we need it to display logs!
	#[cfg(feature = "driver_vga")] {
		vga::init();
	}

	target::env_setup();

	// Initiate core features
	pic::init();
	exception::init();
}
