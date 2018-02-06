// file : cga.zig
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
const mmap= @import("mmap.zig");
const cpu = @import("cpu.zig");

const Color = enum(u4) {
	BLACK = 0x0,
	BLUE = 0x1,
	GREEN = 0x2,
	CYAN = 0x3,
	RED = 0x4,
	MAGENTA = 0x5,
	BROWN = 0x6,
	GRAY = 0x7,
	DARK_GRAY = 0x8,
	BRIGHT_BLUE = 0x9,
	BRIGHT_GREEN = 0xA,
	BRIGHT_CYAN = 0xB,
	BRIGHT_RED = 0xC,
	BRIGHT_MAGENTA = 0xD,
	YELLOW = 0xE,
	WHITE = 0xF,
};

const Entry = packed struct {
	c: u8,
	fg: Color,
	bg: Color,

	pub fn new(c: u8, fg: Color, bg: Color) Entry {
		return Entry{ .c = c, .fg = fg_color, .bg = bg_color };
	}
};

const PORT_CMD: u16 = 0x03D4;
const PORT_DATA: u16 = 0x03D5;

const COLS = 80;
const ROWS = 25;
const TAB_WIDTH = 4;

const vmem = @intToPtr(&volatile Entry, mmap.MEM_CGA_TEXTMODE)[0..COLS * ROWS];
var cursor_pos: u16 = 0;
var fg_color: Color = Color.WHITE;
var bg_color: Color = Color.BLACK;

pub fn init() void {
	cursor_pos = 0;
	fg_color = Color.WHITE;
	bg_color = Color.BLACK;

	// Clean the video memory
	for (vmem) |*entry| {
		*entry = Entry.new(' ', fg_color, bg_color);
	}
}

fn alignLower(index: u16, mod: u16) u16 {
	return index + mod - index % mod;
}

pub fn writeChar(c: u8) void {
	switch (c) {
			// Newline, skip to next row
		'\n' => cursor_pos = alignLower(cursor_pos + 1, COLS),
			// Tab, skip to next tab alignment
		'\t' => cursor_pos = alignLower(cursor_pos + 1, TAB_WIDTH),
			// Printable, display character
		else => if (char.isPrintable(c)) {
				vmem[cursor_pos] = Entry.new(c, fg_color, bg_color);
				cursor_pos += 1;
			},
	}

	if (cursor_pos >= COLS * ROWS)
		cursor_pos = 0;

	setCursorPosition(cursor_pos);
}

pub fn writeStr(str: []const u8) void {
	for (str) |c| {
		writeChar(c);
	}
}

pub fn placeCursor(col: u8, row: u8) void {
	cursor_pos = COLS * row + col;
	setCursorPosition(cursor_pos);
}

pub fn setForeground(col: Color) void {
	fg_color = col;
}

pub fn setBackground(col: Color) void {
	bg_color = col;
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