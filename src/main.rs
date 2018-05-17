// file : main.rs
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
#![feature(repr_transparent)]

// Disable these later
#![allow(dead_code)]
#![feature(core_float)]

extern crate alloc;
extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate cstr_core;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate bitflags;
extern crate arrayvec;
extern crate bimap;

#[macro_use] mod log;
mod shell;

mod llapi;
mod arch;
mod mem;
mod util;
mod thread;
mod process;
mod driver;
mod vfs;
mod fs;
mod fs_;
mod vdev;

use mem::heap::Heap;
#[global_allocator]
pub static HEAP: Heap = Heap::empty();

#[allow(dead_code)]
#[linkage = "external"]
#[no_mangle]
pub extern fn kentry(bootcfg: *const ()) {
	log::init();

	let bootcfg = llapi::bootcfg::parse(bootcfg);

	llapi::cpu::init(&bootcfg);
	llapi::family::init(&bootcfg);
	llapi::chipset::init(&bootcfg);

	loginfo!("Kernel booted with arguments: {:?}", bootcfg.args);

	log::init();
	mem::init(&bootcfg);
	process::init();
	fs::init();
	vfs::init(&bootcfg);
	driver::init();
	vdev::init();

	fs_::init(&bootcfg);

	// Create init process
	// TODO: Make this spawn a process from initramfs
	let init = process::new("init").unwrap_or_else(|e| {
		panic!("Could not spawn init process: {:?}", e);
	});
	logok!("Created init process with uid {}", init.uid());

	// Create init thread
	let init_main = init.spawn_thread("main", shell::main).unwrap_or_else(|e| {
		panic!("Could not spawn init main thread: {:?}", e);
	});
	logok!("Spawned init thread with uid {}", init_main.uid());

	loginfo!("Kernel initiated, waiting for init...");

	// Wait for something to happen
	loop {
		llapi::cpu::irq::enable();
		llapi::cpu::irq::await();
	}
}

// Exception things we don't use
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}
#[no_mangle]
pub extern fn _Unwind_Resume() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(msg: core::fmt::Arguments, file: &'static str, line: u32, column: u32) -> !
{
	unsafe { log::force_unlock(); }
	logln!("\nPANIC: {} in {} on line {} at column {}", msg, file, line, column);
	loop {
		llapi::cpu::irq::disable();
		llapi::cpu::irq::await();
	}
}

#[lang = "oom"]
#[no_mangle]
pub extern fn oom() -> ! {
	panic!("Out Of Memory");
}
