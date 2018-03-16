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
pub mod mem;

#[cfg(feature = "arch_isa_i386")] pub mod i386;
#[cfg(feature = "arch_isa_i386")] pub use arch::x86::i386 as isa;

#[cfg(feature = "arch_isa_x86_64")] pub mod x86_64;
#[cfg(feature = "arch_isa_x86_64")] pub use arch::x86::x86_64 as isa;

// Tag driver
#[cfg(feature = "driver_tags_multiboot")] use driver::tags::multiboot;

pub fn env_setup(tags: *const ()) {
	// Setup TTY out drivers first
	#[cfg(feature = "driver_ttyout_vgatextmode")] {
		use driver::video::vga;
		vga::init()
	}
	#[cfg(feature = "driver_ttyout_com")] {
		use driver::serial::com;
		com::init()
	}

	isa::env_setup();

	// Initiate core features
	use mem;
	mem::init();
	pic::init();
	exception::init();

	// Parse Multiboot data
	#[cfg(feature = "driver_tags_multiboot")] {
		multiboot::init(tags)
	}
}
