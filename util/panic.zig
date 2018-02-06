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

pub fn panicf(comptime format: []const u8, args: ...) noreturn {
	// Set output color to white on red
	tty.setTextColor(tty.Color.WHITE);
	tty.setBackColor(tty.Color.RED);

	// Display the prefix
	fmtCallback({}, "KERNEL PANIC: ")
	catch {
		allIsLost("Displaying panic prefix failed");
	};

	// Display the error message
	fmt.format({}, fmtCallback, format, args)
	catch {
		allIsLost("Displaying panic message failed");
	};

	// Reset output color to defaults
	tty.setTextColor(tty.getDefaultTextColor());
	tty.setBackColor(tty.getDefaultBackColor());

	cpu.hang();
}

fn allIsLost(msg: []const u8) void {
	fmtCallback({}, msg)
	catch {
		// At this point, something has gone majorly wrong
		// Just hang the CPU because clearly nothing else works
		cpu.hang();
	};
}

fn fmtCallback(ctx: void, str: []const u8) %void {
	tty.print(str);
}