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

#[cfg(arch_llapi = "x64")]  use self::x64  as selected;
#[cfg(arch_llapi = "i386")] use self::i386 as selected;
#[cfg(arch_llapi = "rpi2")] use self::rpi2 as selected;

pub use self::selected::meta as meta;
pub use self::selected::cpu as cpu;
pub use self::selected::irq as irq;
pub use self::selected::mem as mem;
pub use self::selected::intrinsic as intrinsic;
