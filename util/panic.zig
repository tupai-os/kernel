// file : panic.zig
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
const tty = @import("../dev/tty.zig");
const cpu = @import("../cpu.zig");

pub fn panicf(comptime format: []const u8, args: ...) void {
	fmt.format({}, fmtCallback, format, args)
	catch {
		fmtCallback({}, "Panic failed")
		catch {
			// At this point, something has gone majorly wrong
			// Just hang the CPU because clearly nothing else works
			cpu.hang();
		};
	};
}

fn fmtCallback(ctx: void, str: []const u8) %void {
	tty.setFgColor(tty.Color.WHITE);
	tty.setBgColor(tty.Color.RED);
	tty.print(str);
	tty.setFgColor(tty.getFgColorDefault());
	tty.setBgColor(tty.getBgColorDefault());
}