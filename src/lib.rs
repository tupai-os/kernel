#![feature(lang_items)]
#![no_std]

#[no_mangle]
pub extern fn kmain() {
	// Nothing yet
	let test = (0..3).flat_map(|x| 0..x).zip(0..);
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! {
	loop {}
}