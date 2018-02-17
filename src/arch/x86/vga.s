// file : vga.s
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

.global _vga_print.boot

.set VGA_BUFFER, 0xB8000
.set VGA_WIDTH, 80
.set VGA_HEIGHT, 25

.section .data.boot
	_cursor:
		.long 0

.section .text.boot
	_vga_print.boot:
		push %edi // We need edi and the System-V ABI specifies that we preserve it
		push %ebp
		mov %esp, %ebp

		// Set up string and cursor pointers
		mov 12(%esp), %ecx
		mov (_cursor), %eax

		1:
			// Find the current character
			movb (%ecx), %dl

			// Is the character \0? End loop if so
			cmp $0, %dl
			je 5f

			// Increment the cursor and write character if it's not a newline character
			cmp $10, %dl
			je 3f
			2:
				mov %dl, VGA_BUFFER(,%eax, 2) // Set char
				movb $0x0F, (VGA_BUFFER + 1)(,%eax, 2) // Set color
				inc %eax // Increment cursor
				jmp 4f
			3:
				mov $0, %edx
				mov $VGA_WIDTH, %edi
				divw %di
				and $0xFF, %eax
				inc %eax
				imul $VGA_WIDTH, %eax
			4:

			// Reset loop
			inc %ecx
			jmp 1b
		5:

		// Write back cursor
		mov %eax, (_cursor)

		pop %ebp
		pop %edi
		ret
