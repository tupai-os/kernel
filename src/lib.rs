#![feature(lang_items)]
#![feature(asm)]
#![no_std]

extern crate rlibc;
extern crate volatile;

pub const VIRT_OFFSET: usize = 0xFFFFFFFF80000000;

pub const VBUFFER: usize = 0xB8000;
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
	use core::mem;
	use volatile::Volatile;

	//let vbuff = unsafe { slice::from_raw_parts_mut(0xB8000 as *mut Volatile<Entry>, COLS * ROWS) };
	let vbuff = unsafe { mem::transmute::<usize, &mut [Volatile<Entry>; COLS * ROWS]>(VIRT_OFFSET + VBUFFER) };

	unsafe {
		vbuff[CURSOR].write(
			Entry {
				c: c,
				fmt: 0xF0
			}
		);
		CURSOR += 1;
	}
}

fn write_str(s: &[u8]) {
	for c in s {
		write_char(*c);
	}
}

#[no_mangle]
pub extern fn kmain(mb_header: *const u32) {
	write_str(b"Hello, World!");
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
