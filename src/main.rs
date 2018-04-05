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

#![no_main]
#![no_std]

#![feature(lang_items)]
#![feature(asm)]
#![feature(ptr_internals)]
#![feature(const_fn)]
#![feature(linkage)]
#![feature(compiler_builtins_lib)]
#![feature(alloc)]
#![feature(global_allocator)]
#![feature(allocator_api)]
#![feature(allocator_internals)]
#![feature(global_asm)]

// Disable these later
#![allow(dead_code)]
#![feature(core_float)]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate compiler_builtins;
#[macro_use]
extern crate lazy_static;
extern crate cstr_core;
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate bitflags;

#[macro_use]
mod log;
mod arch;
mod util;
mod mem;
mod process;
mod driver;
mod llapi;

use mem::heap::Heap;
#[global_allocator]
pub static HEAP: Heap = Heap::empty();

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
pub extern fn kmain(args: &[&str]) {
	logln!("Kernel booted with arguments: {:?}", args);

	// Wait for something to happen
	llapi::irq::enable();
	loop {
		llapi::cpu::halt();
	}
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
	log!("PANIC: {} in {} on line {} at column {}", msg, file, line, column);
	loop {
		llapi::irq::disable();
		llapi::cpu::halt();
	}
}
