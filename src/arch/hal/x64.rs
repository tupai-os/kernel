// file : mod.rs
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

// pub struct Hal {}
// impl super::HalTrait for Hal {
//	 type Cpu = Cpu;
//	 type Irq = Irq;
//	 type VirtMemCtx = VirtMemCtx;
// }

// pub struct Cpu {}
// impl super::CpuTrait for Cpu {
//	 fn halt() {
//		 unsafe { asm!("hlt") }
//	 }
// }

// pub struct Irq {}
// impl super::IrqTrait for Irq {
//	 type ExceptionFrame = ExceptionFrame;
//	 type InterruptFrame = InterruptFrame;

//	 fn enable() {
//		 unsafe { asm!("sti") }
//	 }

//	 fn disable() {
//		 unsafe { asm!("cli") }
//	 }
// }

// pub struct VirtMemCtx {}
// impl super::VirtMemCtxTrait for VirtMemCtx {
//	 fn map(virt_page: usize, phys_page: usize) {
//		 // TODO: Implement this
//	 }

//	 fn try_map(virt_page: usize, phys_page: usize) -> bool {
//		 false // TODO: Implement this
//	 }

//	 fn get_map(virt_page: usize) -> Option<usize> {
//		 Some(0) // TODO: Implement this
//	 }
// }

// pub struct ExceptionFrame {}
// impl super::StackFrameTrait for ExceptionFrame {
//	 fn get_code_addr() -> usize {
//		 0 // TODO: Implement this
//	 }

//	 fn get_stack_addr() -> usize {
//		 0 // TODO: Implement this
//	 }
// }

// pub struct InterruptFrame {}
// impl super::StackFrameTrait for InterruptFrame {
//	 fn get_code_addr() -> usize {
//		 0 // TODO: Implement this
//	 }

//	 fn get_stack_addr() -> usize {
//		 0 // TODO: Implement this
//	 }
// }

pub mod cpu {
	use arch::isa::amd64;

	pub use self::amd64::halt;
}

pub mod irq {
	use arch::isa::amd64;

	pub use self::amd64::enable_irqs as enable;
	pub use self::amd64::disable_irqs as disable;
}

pub mod mem {
	use arch::isa::amd64;

	pub use self::amd64::PAGE_SIZE_KB_LOG2;
	pub use self::amd64::mem::PageMap;
}
