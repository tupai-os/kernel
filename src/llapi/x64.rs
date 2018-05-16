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

pub use arch::cpu::amd64         as cpu;
pub use arch::family::x86        as family;
pub use arch::chipset::ibmpc     as chipset;
pub use arch::bootcfg::multiboot as bootcfg;

pub mod driver {
	use driver;

	pub const ON_BOOT: [&'static driver::Desc; 2] = [
		&driver::com::DESC,
		&driver::console::DESC,
	];

	pub const LOG: &'static driver::Desc = &driver::console::DESC;
}

pub const fn name() -> &'static str { "x64" }
