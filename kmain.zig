// file : kmain.zig
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

const vbuff = @intToPtr(&volatile u16, 0xB8000)[0..0x4000];
var cursor: u16 = 0;

fn print(txt: []const u8) void {
	for (txt) |c| {
		vbuff[cursor] = (0x0F << 8) | u16(c);
		cursor += 1;
	}
}

export fn kmain() void {
	print("Hello, World!");
	asm volatile ("hlt");
}