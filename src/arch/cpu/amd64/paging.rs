// file : paging.rs
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

// Invariant constants
pub const PAGE_KB_LOG2: usize = 2;
pub const PAGE_B_LOG2: usize = 10 + PAGE_KB_LOG2;

// Variant constants
pub const VIRTUAL_OFFSET: usize = 0xFFFFFFFF80000000;

pub struct PageMap {
	// Nothing yet
}

impl PageMap {
	pub fn new() -> PageMap {
		PageMap {}
	}
}
