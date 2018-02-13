#![feature(lang_items)]
#![feature(asm)]
#![no_std]

extern crate volatile;

use core::mem;

pub const COLS: usize = 80;
pub const ROWS: usize = 25;

pub static mut CURSOR: usize = 0;

#[repr(packed)]
#[derive(Copy, Clone)]
struct Entry {
	c: u8,
	fmt: u8,
}

fn write_char(c: u8) {
	use volatile::Volatile;

	//let vbuff = unsafe { slice::from_raw_parts_mut(0xB8000 as *mut Volatile<Entry>, COLS * ROWS) };
	let vbuff = unsafe { mem::transmute::<usize, &mut [Volatile<Entry>; COLS * ROWS]>(0xB8000) };

	unsafe {
		vbuff[CURSOR].write(
			Entry {
				c: c,
				fmt: 0x0F
			}
		);
		CURSOR += 1;
	}
}

#[no_mangle]
pub extern fn kmain() {
	for c in b"Hello, World!" {
		write_char(*c);
	}

	loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! {
	loop {}
}