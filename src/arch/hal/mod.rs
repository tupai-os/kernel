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

// pub trait HalTrait {
//	 type Cpu: CpuTrait;
//	 type Irq: IrqTrait;
//	 type VirtMemCtx: VirtMemCtxTrait;
// }

// pub trait CpuTrait {
//	 fn halt();
// }

// pub trait IrqTrait {
//	 type ExceptionFrame: StackFrameTrait;
//	 type InterruptFrame: StackFrameTrait;

//	 fn enable();
//	 fn disable();
// }

// pub trait StackFrameTrait {
//	 fn get_code_addr() -> usize;
//	 fn get_stack_addr() -> usize;
// }

// pub trait VirtMemCtxTrait {
//	 fn map(virt_page: usize, phys_page: usize);
//	 fn try_map(virt_page: usize, phys_page: usize) -> bool;
//	 fn get_map(virt_page: usize) -> Option<usize>;
// }

#[cfg(arch_hal = "i386")]  pub mod i386;
#[cfg(arch_hal = "x64")]   pub mod x64;
#[cfg(arch_hal = "armv7")] pub mod armv7;
#[cfg(arch_hal = "armv8")] pub mod armv8;

// Export selected HAL module
#[cfg(arch_hal = "i386")]  pub use self::i386  as selected;
#[cfg(arch_hal = "x64")]   pub use self::x64   as selected;
#[cfg(arch_hal = "armv7")] pub use self::armv7 as selected;
#[cfg(arch_hal = "armv8")] pub use self::armv8 as selected;
