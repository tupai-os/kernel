// file : check.s
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

.extern _vga_print.boot
.extern _mb_magic.boot
.extern _mb_header.boot
.extern _hang.boot

.global _check.boot

.set MB_VALID_MAGIC, 0x36D76289

.section .rodata.boot
	_mb_msg_good:
		.ascii "[ OK ] Multiboot magic is valid\n\0"
	_mb_msg_bad:
		.ascii "[FAIL] Multiboot magic is invalid\n\0"

.section .text.boot
	_check.boot:
		push %ebp
		mov %esp, %ebp

		cmpl $MB_VALID_MAGIC, (_mb_magic.boot)
		je 2f
		1:
			// Multiboot magic failure
			push $_mb_msg_bad
			call _vga_print.boot

			jmp _hang.boot
		2:
			// Multiboot magic success
			push $_mb_msg_good
			call _vga_print.boot

		pop %ebp
		ret
