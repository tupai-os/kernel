// file : char.zig
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

const vmem = @intToPtr(&volatile u16, 0xB8000)[0..0x4000];
var cursor: u16 = 0;

pub fn isWhitespace(c: u8) bool {
	return switch (c) {
		' ', '\t', '\n, '\r', '\v' => true,
		else => false,
	};
}

pub fn isPrintable(c: u8) bool {
	return switch (c) {
		' ' => true,
		'!' ... '/' => true,
		'0' ... '9' => true,
		':' ... '@' => true,
		'A' ... 'Z' => true,
		'[' ... '`' => true,
		'a' ... 'z' => true,
		'{' ... '~' => true,
		else => false,
	};
}