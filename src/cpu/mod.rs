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

#[cfg(feature = "arch_family_x86")] use arch::x86;
#[cfg(feature = "arch_family_arm")] use arch::arm;

pub fn enable_irqs() {
	#[cfg(feature = "arch_family_x86")] {
		x86::cpu::enable_irqs();
	}

	#[cfg(feature = "arch_family_arm")] {
		arm::cpu::enable_irqs();
	}
}


pub fn halt() {
	#[cfg(feature = "arch_family_x86")] {
		x86::cpu::halt();
	}

	#[cfg(feature = "arch_family_arm")] {
		arm::cpu::halt();
	}
}
