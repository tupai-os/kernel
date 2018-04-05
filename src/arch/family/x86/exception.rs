// file : exception.rs
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

use llapi::intrinsic::isa::idt;
use llapi::intrinsic::isa::isr;
use llapi::cpu;

extern {
	fn _exception_handler0();
	fn _exception_handler1();
	fn _exception_handler2();
	fn _exception_handler3();
	fn _exception_handler4();
	fn _exception_handler5();
	fn _exception_handler6();
	fn _exception_handler7();
	fn _exception_handler8();
	fn _exception_handler9();
	fn _exception_handler10();
	fn _exception_handler11();
	fn _exception_handler12();
	fn _exception_handler13();
	fn _exception_handler14();
	// <Reserved>
	fn _exception_handler16();
	fn _exception_handler17();
	fn _exception_handler18();
	fn _exception_handler19();
	fn _exception_handler20();
	// <Reserved>
	fn _exception_handler30();
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum Exception {
	DivZero          = 0,
	Debug            = 1,
	NonMaskable      = 2,
	Breakpoint       = 3,
	Overflow         = 4,
	BoundRange       = 5,
	InvalidOpcode    = 6,
	NoFPU            = 7,
	DoubleFault      = 8,
	FPUSegFault      = 9,
	InvalidTSS       = 10,
	SegNotPresent    = 11,
	StackSegFault    = 12,
	GProtectionFault = 13,
	PageFault        = 14,
	// <Reserved>
	x87Error         = 16,
	AlignmentCheck   = 17,
	MachineCheck     = 18,
	SIMDError        = 19,
	VirtError        = 20,
	// <Reserved>
	SecurityError    = 30,
	// <Reserved>
}

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
extern fn divzero_handler(frame: *mut isr::ExceptionFrame) {
	exception_panic(Exception::DivZero, unsafe { &*frame });
}

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
extern fn debug_handler(frame: *mut isr::ExceptionFrame) {
	exception_panic(Exception::Debug, unsafe { &*frame });
}

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
extern fn segnotpresent_handler(frame: *mut isr::ExceptionFrame) {
	exception_panic(Exception::SegNotPresent, unsafe { &*frame });
}

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
extern fn gprotectionfault_handler(frame: *mut isr::ExceptionFrame) {
	exception_panic(Exception::GProtectionFault, unsafe { &*frame });
}

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
extern fn pagefault_handler(frame: *mut isr::ExceptionFrame) {
	exception_panic(Exception::PageFault, unsafe { &*frame });
}

#[no_mangle]
#[allow(dead_code)]
#[linkage = "external"]
extern fn unimplemented_handler(frame: *mut isr::ExceptionFrame) {
	logln!("Unimplemented exception occured");
	logln!("Machine state:\n{}", unsafe { &*frame });
}

fn exception_panic(ex: Exception, frame: &isr::ExceptionFrame) {
	logln!("'{}' exception occured", match ex {
		Exception::DivZero => "Divide by Zero",
		Exception::Debug => "Debug",
		Exception::SegNotPresent => "Segment Not Present",
		Exception::GProtectionFault => "General Protection Fault",
		Exception::PageFault => "Page Fault",
		_ => "Unimplemented",
	});
	logln!("Machine state:\n{}", frame);

	loop {
		cpu::halt();
	}
}

pub fn init() {
	idt::set_handler(0, _exception_handler0 as idt::IsrPtr);
	idt::set_handler(1, _exception_handler1 as idt::IsrPtr);
	idt::set_handler(2, _exception_handler2 as idt::IsrPtr);
	idt::set_handler(3, _exception_handler3 as idt::IsrPtr);
	idt::set_handler(4, _exception_handler4 as idt::IsrPtr);
	idt::set_handler(5, _exception_handler5 as idt::IsrPtr);
	idt::set_handler(6, _exception_handler6 as idt::IsrPtr);
	idt::set_handler(7, _exception_handler7 as idt::IsrPtr);
	idt::set_handler(8, _exception_handler8 as idt::IsrPtr);
	idt::set_handler(9, _exception_handler9 as idt::IsrPtr);
	idt::set_handler(10, _exception_handler10 as idt::IsrPtr);
	idt::set_handler(11, _exception_handler11 as idt::IsrPtr);
	idt::set_handler(12, _exception_handler12 as idt::IsrPtr);
	idt::set_handler(13, _exception_handler13 as idt::IsrPtr);
	idt::set_handler(14, _exception_handler14 as idt::IsrPtr);
	// <Reserved>
	idt::set_handler(16, _exception_handler16 as idt::IsrPtr);
	idt::set_handler(17, _exception_handler17 as idt::IsrPtr);
	idt::set_handler(18, _exception_handler18 as idt::IsrPtr);
	idt::set_handler(19, _exception_handler19 as idt::IsrPtr);
	idt::set_handler(20, _exception_handler20 as idt::IsrPtr);
	// <Reserved>
	idt::set_handler(30, _exception_handler30 as idt::IsrPtr);

	idt::reinstall();

	// TODO: Fix this
	logok!("Set exception handlers");
}
