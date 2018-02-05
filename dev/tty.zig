// file : tty.zig
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

const fmt = @import("std").fmt;
const arch = @import("../arch.zig");
const vga = if (arch.is_x86_family()) @import("../arch/x86/vga.zig");

const COLS = 80;
const ROWS = 25;

const vmem = @intToPtr(&volatile u16, 0xB8000)[0..0x4000];
var cursor: u16 = 0;

pub fn print(str: []const u8) void {
	vga.writeStr(str);
}

pub fn printf(format: []const u8, args: ...) void {
	fmt.format({}, fmtCallback, format, args);
}

fn fmtCallback(ctx: void, str: []const u8) %void {
	vga.writeStr(str);
}