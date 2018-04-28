// file : i386.rs
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

pub mod cpu {
	pub use arch::isa::ia32::halt;
}

pub mod irq {
	pub use arch::isa::ia32::enable_irqs as enable;
	pub use arch::isa::ia32::disable_irqs as disable;

	pub use arch::isa::ia32::isr::InterruptFrame;
}

pub mod mem {
	pub use arch::isa::ia32::mem::PAGE_SIZE_KB_LOG2;
	pub use arch::isa::ia32::mem::VMEMORY_OFFSET;
	pub use arch::isa::ia32::mem::PageMap;
}

pub mod intrinsic {
	pub use arch::isa::ia32 as isa;
	pub use arch::family::x86 as family;
	pub use arch::chipset::ibmpc as chipset;
}
