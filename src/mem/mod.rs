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

use llapi::cpu::paging::{PAGE_KB_LOG2, PAGE_B_LOG2};
use util::bootcfg::BootCfg;
use util::{
	elf::kernel_bounds,
	math::{kb_to_page, addr_to_page, align_up},
};
use process::ProcessHandle;

pub fn init(bootcfg: &BootCfg) {
	wma::init();
	pfa::init();
	heap::init();

	pfa::set_range(0, kb_to_page(align_up(bootcfg.mem_ram as usize, PAGE_KB_LOG2)), pfa::ENTRY_FREE_RAM).unwrap_or_else(|e|{
		panic!("Could not reserve available RAM from {:X}K to {:X}K ({:?})", 0, bootcfg.mem_ram, e);
	});
	logok!("Reserved available RAM from {:X}K to {:X}K", 0, bootcfg.mem_ram);

	pfa::set_range(
		0,
		addr_to_page(align_up(kernel_bounds().end, PAGE_B_LOG2)),
		pfa::PageEntry::new(
			ProcessHandle::kernel(),
			pfa::Flags::RAM | pfa::Flags::USED | pfa::Flags::STATIC // Used, immovable RAM
		)
	).unwrap_or_else(|e|{
		panic!("Could not reserve kernel memory from {:X}K to {:X}K ({:?})",
			0,
			addr_to_page(align_up(kernel_bounds().end, PAGE_B_LOG2)),
			e
		);
	});
	logok!("Reserved kernel memory");

	logok!("Initiated memory management");
}
