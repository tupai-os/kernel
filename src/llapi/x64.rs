// file : x64.rs
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

use arch;
use super::Llapi;

// pub mod meta {
//	 pub const VARIANT: &str = "x64";
//	 pub const FAMILY: &str = "x86";
//	 pub const ISA: &str = "amd64";
//	 pub const CHIPSET: &str = "ibmpc";
// }

// pub mod cpu {
//	 pub use arch::isa::amd64::halt;
// }

// pub mod irq {
//	 pub use arch::isa::amd64::irq_enable as enable;
//	 pub use arch::isa::amd64::irq_disable as disable;
//	 pub use arch::isa::amd64::irq_enabled as enabled;

//	 pub use arch::isa::amd64::isr::StackFrame;
// }

// pub mod mem {
//	 pub use arch::isa::amd64::mem::PAGE_SIZE_KB_LOG2;
//	 pub use arch::isa::amd64::mem::PAGE_SIZE_LOG2;
//	 pub use arch::isa::amd64::mem::VMEMORY_OFFSET;
//	 pub use arch::isa::amd64::mem::PageMap;
// }

// pub mod intrinsic {
//	 pub use arch::isa::amd64 as isa;
//	 pub use arch::family::x86 as family;
//	 pub use arch::chipset::ibmpc as chipset;
// }

#[allow(non_camel_case_types)]
pub struct x64 {}

impl Llapi for x64 {
	type family = arch::family::X86;
	type cpu = arch::cpu::Amd64;
	type chipset = <arch::chipset::IbmPc as Trait>;

	fn name() -> &'static str { "x64" }
}
