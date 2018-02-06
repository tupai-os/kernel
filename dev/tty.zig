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
const panic = @import("../util/panic.zig");
const cga = if (arch.is_x86_family()) @import("../arch/x86/cga.zig");

pub const Color = if (arch.is_x86_family()) @import("../arch/x86/cga.zig").Color;

pub fn print(str: []const u8) void {
	cga.writeStr(str);
}

pub fn printf(comptime format: []const u8, args: ...) void {
	fmt.format({}, fmtCallback, format, args)
	catch {
		panic.panicf("Printf failed");
	};
}

fn fmtCallback(ctx: void, str: []const u8) %void {
	if (arch.is_x86_family()) {
		cga.writeStr(str);
	}
}

pub fn getFgColorDefault() Color {
	if (arch.is_x86_family()) {
		return cga.getFgColorDefault();
	} else {
		return Color.WHITE;
	}
}

pub fn getBgColorDefault() Color {
	if (arch.is_x86_family()) {
		return cga.getBgColorDefault();
	} else {
		return Color.BLACK;
	}
}

pub fn setFgColor(col: Color) void {
	if (arch.is_x86_family()) {
		cga.setFgColor(col);
	}
}

pub fn setBgColor(col: Color) void {
	if (arch.is_x86_family()) {
		cga.setBgColor(col);
	}
}