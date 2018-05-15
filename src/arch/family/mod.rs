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

#[cfg(arch_family = "x86")] mod x86;
#[cfg(arch_family = "arm")] mod arm;

#[cfg(arch_family = "x86")] pub use x86::X86 as X86;
#[cfg(arch_family = "arm")] pub use arm::Arm as Arm;

pub trait Family {
	fn init();

	fn name() -> &'static str;
}
