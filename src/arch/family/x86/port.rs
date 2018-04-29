// file : port.rs
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

#![allow(dead_code)]

// TODO: Improve this
pub fn wait(_cycles: usize) {
	use volatile::Volatile;
 	let mut i: usize = 0;
	let iv = unsafe { &mut *(&mut i as *mut usize as *mut Volatile<usize>) };
	while iv.read() < _cycles {
		let old = iv.read();
		iv.write(old + 1);
	}
}

// Output

pub fn out8(port: u16, value: u8) {
	unsafe { asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile"); }
	wait(150);
}

pub fn out16(port: u16, value: u16) {
	unsafe { asm!("outb %ax, %dx" :: "{dx}"(port), "{ax}"(value) :: "volatile"); }
	wait(150);
}

pub fn out32(port: u16, value: u32) {
	unsafe { asm!("outb %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile"); }
	wait(150);
}

// Rapid port I/O without a delay

pub fn fast_out8(port: u16, value: u8) {
	unsafe { asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile"); }
}

pub fn fast_out16(port: u16, value: u16) {
	unsafe { asm!("outb %ax, %dx" :: "{dx}"(port), "{ax}"(value) :: "volatile"); }
}

pub fn fast_out32(port: u16, value: u32) {
	unsafe { asm!("outb %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile"); }
}

// Input

pub fn in8(port: u16) -> u8 {
	let val: u8;
	wait(150);
	unsafe { asm!("inb %dx, %al" : "={al}"(val) : "{dx}"(port) :: "volatile"); }
	val
}

pub fn in16(port: u16) -> u16 {
	let val: u16;
	wait(150);
	unsafe { asm!("inw %dx, %ax" : "={ax}"(val) : "{dx}"(port) :: "volatile"); }
	val
}

pub fn in32(port: u16) -> u32 {
	let val: u32;
	wait(150);
	unsafe { asm!("inl %dx, %eax" : "={eax}"(val) : "{dx}"(port) :: "volatile"); }
	val
}

// Rapid port I/O without delay

pub fn fast_in8(port: u16) -> u8 {
	let val: u8;
	unsafe { asm!("inb %dx, %al" : "={al}"(val) : "{dx}"(port) :: "volatile"); }
	val
}

pub fn fast_in16(port: u16) -> u16 {
	let val: u16;
	unsafe { asm!("inw %dx, %ax" : "={ax}"(val) : "{dx}"(port) :: "volatile"); }
	val
}

pub fn fast_in32(port: u16) -> u32 {
	let val: u32;
	unsafe { asm!("inl %dx, %eax" : "={eax}"(val) : "{dx}"(port) :: "volatile"); }
	val
}
