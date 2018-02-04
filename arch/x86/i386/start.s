// file : start.s
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

.extern kmain
.global _start.boot

.section .bss.boot
	.align 16
	_stack_start.boot:
		.skip 4096 // 4K stack
	_stack_end.boot:

.section .text.boot
	.type _start.boot, @function
	_start.boot:
		mov $_stack_end.boot, %esp // Set stack

		call kmain // Call kernel main

		_hang.boot: // Hang the kernel
			cli
			hlt
			jmp _hang.boot