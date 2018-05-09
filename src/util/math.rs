// file : math.rs
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

use llapi::mem::PAGE_SIZE_KB_LOG2;

pub fn align_down(x: usize, log2: usize) -> usize {
	x & (!0 << log2)
}

pub fn align_up(x: usize, log2: usize) -> usize {
	(x + (1 << log2) - 1) & (!0 << log2)
}

pub fn addr_to_page(addr: usize) -> usize {
	addr >> (PAGE_SIZE_KB_LOG2 + 10)
}

pub fn kb_to_page(kb: usize) -> usize {
	kb >> PAGE_SIZE_KB_LOG2
}
