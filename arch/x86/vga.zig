// file : vga.zig
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

const char = @import("../../util/char.zig");

const COLS = 80;
const ROWS = 25;

const vmem = @intToPtr(&volatile u16, 0xB8000)[0..0x4000];
var cursor: u16 = 0;

pub fn write_char(c: u8) void {
	switch (c) {
		'\n' => {
				while (cursor % COLS > 0)
					write_char(' ');
			},
		else => {
				vmem[cursor] = (0x0F << 8) | u16(c);
				cursor += 1;
			},
	}
}

pub fn write_str(str: []const u8) void {
	for (str) |c| {
		write_char(c);
	}
}