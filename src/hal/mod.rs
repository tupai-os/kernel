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

#[cfg(arch_hal = "i386")]  mod i386;
#[cfg(arch_hal = "amd64")] mod amd64;
#[cfg(arch_hal = "armv7")] mod armv7;
#[cfg(arch_hal = "armv8")] mod armv8;

#[cfg(arch_hal = "i386")]  pub use self::i386::*;
#[cfg(arch_hal = "amd64")] pub use self::amd64::*;
#[cfg(arch_hal = "armv7")] pub use self::armv7::*;
#[cfg(arch_hal = "armv8")] pub use self::armv8::*;
