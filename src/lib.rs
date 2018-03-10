// file : lib.rs
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

#![feature(lang_items)]
#![feature(asm)]
#![feature(ptr_internals)]
#![feature(const_fn)]
#![feature(linkage)]
#![feature(compiler_builtins_lib)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate compiler_builtins;
#[macro_use]
extern crate lazy_static;

#[macro_use] mod util;
mod arch;
mod cpu;
mod mem;
mod driver;

#[no_mangle]
pub extern fn kmain(tags: *const ()) {
	// Setup arch-specific things
	arch::base::env_setup(tags);

	// Setup memory management
	mem::init();

	loginfo!("Entered kernel main");

	logln!("Welcome to the main kernel!");

	// Wait for something to happen
	cpu::enable_irqs();
	cpu::halt();
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[no_mangle]
pub extern fn _Unwind_Resume() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(msg: core::fmt::Arguments, file: &'static str, line: u32, column: u32) -> !
{
	logln!("Panic in {} on line {} at column {}:\n{}", file, line, column, msg);
    loop {
		cpu::disable_irqs();
		cpu::halt();
	}
}
