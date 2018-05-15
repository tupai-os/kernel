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

#[cfg(arch_cpu = "amd64")] mod amd64;
#[cfg(arch_cpu = "ia32")]  mod ia32;
#[cfg(arch_cpu = "a32")]   mod a32;
#[cfg(arch_cpu = "a64")]   mod a64;

#[cfg(arch_cpu = "amd64")] pub use amd64::Amd64 as Amd64;
#[cfg(arch_cpu = "ia32")]  pub use ia32::Ia32 as Ia32;
#[cfg(arch_cpu = "a32")]   pub use a32::A32 as A32;
#[cfg(arch_cpu = "a64")]   pub use a64::A64 as A64;

pub trait Cpu {
	#[allow(non_camel_case_types)]
	type irq: Irq;

	fn name() -> &'static str;
}

pub trait Irq {
	fn enable();
	fn disable();
	fn is_enabled() -> bool;
}
