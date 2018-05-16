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

// Publicly visible API
pub mod irq;
pub mod paging;

pub mod intrinsic {
	pub use super::{gdt, idt};
}

// TODO: Make these private
mod boot;
pub mod gdt;
pub mod idt;
pub mod isr;

global_asm!(include_str!("isr.s"));

use util::bootcfg::BootCfg;

pub const fn name() -> &'static str { "amd64" }

pub fn init(_bootcfg: &BootCfg) {
	gdt::init();
	idt::init();
}
