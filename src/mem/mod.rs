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

pub mod wma;
pub mod pfa;
pub mod heap;

use arch::tags::BootData;

pub fn init(boot_data: &BootData) {
	wma::init();
	pfa::init();
	heap::init();

	pfa::set_range_kb(0, boot_data.mem_ram as usize, pfa::ENTRY_FREE_RAM).unwrap_or_else(|e|{
		panic!("Could not reserve available RAM from {:X}K to {:X}K ({:?})", 0, boot_data.mem_ram, e);
	});
	logok!("Reserved available RAM");

	logok!("Initiated memory management");
}
