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
#![feature(alloc)]
#![feature(global_allocator)]
#![feature(allocator_api)]
#![feature(allocator_internals)]
#![no_std]

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

#[macro_use] mod util;
mod arch;
mod cpu;
mod mem;
mod driver;
mod env;

use mem::heap::Heap;
#[global_allocator]
pub static HEAP: Heap = Heap::empty();

#[no_mangle]
pub extern fn kmain(tags: *const ()) {
	// Initiate arch-specific things
	arch::base::init(tags);

	// Initiate core systems
	env::init();

	// Create kernel environment
	let kernel_env = env::create("kernel", env::Flags::KERNEL)
		.expect("Failed to create kernel environment");
	logok!("Created kernel environment");
	use util::elf::kernel_bounds;
	use mem::pfa::{PageEntry, Flags, set_range};
	match set_range(
		kernel_bounds().start,
		kernel_bounds().end,
		PageEntry::new(kernel_env, Flags::RAM | Flags::USED)
	) {
		Ok(_) => logok!("Reserved kernel from {:p} to {:p}", kernel_bounds().start as *const (), kernel_bounds().end as *const ()),
		Err(e) => panic!("Could not reserve kernel memory: {:?}", e),
	}

	// Initiate drivers
	driver::init();
	loginfo!("Initiated drivers");

	loginfo!("Kernel initiated");

	mem::pfa::display();

	// Wait for something to happen
	loginfo!("Initiation completed, waiting for scheduler...");
	cpu::enable_irqs();
	loop {
		cpu::halt()
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
	logln!("Panic in {} on line {} at column {}:\n{}", file, line, column, msg);
    loop {
		cpu::disable_irqs();
		cpu::halt();
	}
}
