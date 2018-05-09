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

#[cfg(arch_tags = "multiboot")] pub mod multiboot;
#[cfg(arch_tags = "atags")]     pub mod atags;

use arrayvec::ArrayVec;
use core::default::Default;

type Args = ArrayVec<[&'static str; 64]>; // Max 64 args

pub struct Module {
	pub start: usize,
	pub end: usize,
	pub cmd: &'static str,
}

#[derive(Default)]
pub struct BootData {
	pub args: Args,
	pub mem_ram: usize,
	pub modules: ArrayVec<[Module; 4]>, // Max 4 modules
}

impl Module {
	pub fn new(start: usize, end: usize, cmd: &'static str) -> Module {
		Module {
			start: start,
			end: end,
			cmd: cmd,
		}
	}
}

impl BootData {
	pub fn empty() -> BootData {
		return Default::default();
	}
}
