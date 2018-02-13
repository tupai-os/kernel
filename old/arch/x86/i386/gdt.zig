// file : gdt.zig
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

const Entry = packed struct {
	size_lo: u16,
	offset_lo: u16,
	offset_mid: u8,
	access: u8,
	size_hi: u4,
	flags: u4,
	offset_hi: u8,
};

const GDT_LEN = 5;
var gdt align(4): [GDT_LEN; u8];