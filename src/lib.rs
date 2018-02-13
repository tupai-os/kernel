#![feature(lang_items)]
#![no_std]

#[no_mangle]
pub extern fn kmain() {
	// Nothing yet
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! {
	loop {}
}