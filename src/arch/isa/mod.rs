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

#[cfg(arch_isa = "ia32")]  pub mod ia32;
#[cfg(arch_isa = "amd64")] pub mod amd64;
#[cfg(arch_isa = "a32")]   pub mod a32;
#[cfg(arch_isa = "a64")]   pub mod a64;

// Export selected ISA module
#[cfg(arch_isa = "ia32")]  pub use self::ia32  as selected;
#[cfg(arch_isa = "amd64")] pub use self::amd64 as selected;
#[cfg(arch_isa = "a32")]   pub use self::a32   as selected;
#[cfg(arch_isa = "a64")]   pub use self::a64   as selected;
