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

#[cfg(arch_llapi = "x64")]  mod x64;
#[cfg(arch_llapi = "i386")] mod i386;
#[cfg(arch_llapi = "rpi2")] mod rpi2;

#[cfg(arch_llapi = "x64")]  pub use self::x64::x64  as Selected;
#[cfg(arch_llapi = "i386")] pub use self::i386::i386 as Selected;
#[cfg(arch_llapi = "rpi2")] pub use self::rpi2::rpi2 as Selected;

use arch::{
	family::Family,
	cpu::Cpu,
	chipset::Chipset,
};

pub trait Llapi {
	#[allow(non_camel_case_types)]
	type family: Trait; // Family;
	#[allow(non_camel_case_types)]
	type cpu: Trait; // Cpu;
	#[allow(non_camel_case_types)]
	type chipset: Trait; // Chipset;

	fn name() -> &'static str;
}
