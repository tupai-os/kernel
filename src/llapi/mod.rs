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

pub use self::selected::*;

// Contract checking
// TODO: Remove this when/if Rust gains some kind of contract assurance

use util::contract::ensure_same;
use util::bootcfg::BootCfg;

#[allow(dead_code)]
fn ensure_contract() {
	// LLAPI
	{
		ensure_same::<fn() -> &'static str>(name);
	}

	// Family
	{
		ensure_same::<fn() -> &'static str>(family::name);
		ensure_same::<fn(bootcfg: &BootCfg)>(family::init);
	}

	// CPU
	{
		ensure_same::<fn() -> &'static str>(cpu::name);
		ensure_same::<fn(bootcfg: &BootCfg)>(cpu::init);

		// IRQ
		{
			ensure_same::<fn()>(cpu::irq::enable);
			ensure_same::<fn()>(cpu::irq::disable);
			ensure_same::<fn()>(cpu::irq::await);
			ensure_same::<fn() -> bool>(cpu::irq::is_enabled);
		}
	}

	// Chipset
	{
		ensure_same::<fn() -> &'static str>(chipset::name);
		ensure_same::<fn(bootcfg: &BootCfg)>(chipset::init);
	}

	// BootCfg
	{
		ensure_same::<fn() -> &'static str>(bootcfg::name);
	}
}
