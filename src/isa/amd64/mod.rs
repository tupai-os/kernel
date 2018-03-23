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

pub mod idt;
pub mod gdt;
pub mod isr;
pub mod cpu;

global_asm!(include_str!("boot/check.s"));
global_asm!(include_str!("boot/high.s"));
global_asm!(include_str!("boot/paging.s"));
global_asm!(include_str!("boot/paging64.s"));
global_asm!(include_str!("boot/vga.s"));
global_asm!(include_str!("boot/start.s"));

pub fn init() {
	// Nothing yet
}