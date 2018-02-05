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
const cpu = @import("cpu.zig");

const COLS = 80;
const ROWS = 25;

const Entry = packed struct {
	c: u8,
	fg: u4,
	bg: u4,
};

const PORT_CMD: u16 = 0x03D4;
const PORT_DATA: u16 = 0x03D5;

const vmem = @intToPtr(&volatile Entry, 0xB8000)[0..0x4000];
var cursor: u16 = 0;

pub fn writeChar(c: u8) void {
	switch (c) {
		'\n' => {
				while (cursor % COLS > 0)
					cursor += 1;
			},
		else => {
				vmem[cursor] = Entry{ .c = c, .fg = 0xF, .bg = 0x0 };
				cursor += 1;

				if (cursor >= COLS * ROWS)
					cursor = 0;
			},
	}

	setCursorPosition(cursor);
}

pub fn writeStr(str: []const u8) void {
	for (str) |c| {
		writeChar(c);
	}
}

pub fn moveTo(col: u8, row: u8) void {
	cursor = COLS * row + col;
	setCursorPosition(cursor);
}

pub fn setCursorEnabled(comptime enabled: bool) void {
	if (enabled) {
		cpu.out8(PORT_CMD, 0x0A);
		cpu.out8(PORT_DATA, 0x00);
	}
	else {
		cpu.out8(PORT_CMD, 0x0A);
		cpu.out8(PORT_DATA, 0x3F);
	}
}

pub fn setCursorPosition(offset: u16) void {
	cpu.out8(PORT_CMD, 0x0F);
	cpu.out8(PORT_DATA, u8(offset & 0xFF));
	cpu.out8(PORT_CMD, 0x0E);
	cpu.out8(PORT_DATA, u8((offset >> 8) & 0xFF));
}