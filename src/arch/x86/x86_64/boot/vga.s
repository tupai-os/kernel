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

.extern _vga_cursor.boot

.global _vga_print64.boot

.set VGA_BUFFER, 0xB8000
.set VGA_WIDTH, 80
.set VGA_HEIGHT, 25

.code64
.section .text.boot
	_vga_print64.boot:
		push %rbp
		mov %rsp, %rbp

		// Set up cursor (rdi already contains char pointer)
		mov (_vga_cursor.boot), %rax

		1:
			// Find the current character
			movb (%rdi), %dl

			// Is the character \0? End loop if so
			cmp $0, %dl
			je 5f

			// Increment the cursor and write character if it's not a newline character
			cmp $10, %dl
			je 3f
			2:
				mov %dl, VGA_BUFFER(,%rax, 2) // Set char
				movb $0x0F, (VGA_BUFFER + 1)(,%rax, 2) // Set color
				inc %rax // Increment cursor
				jmp 4f
			3:
				mov $0, %rdx
				mov $VGA_WIDTH, %rsi
				divw %si
				and $0xFF, %rax
				inc %rax
				imul $VGA_WIDTH, %rax
			4:

			// Reset loop
			inc %rdi
			jmp 1b
		5:

		// Write back cursor
		mov %rax, (_vga_cursor.boot)

		pop %rbp
		retq
