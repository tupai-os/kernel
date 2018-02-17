// file : high.s
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
.extern kmain

.global _start
.global _hang

.set VIRTUAL_OFFSET, 0xC0000000

.section .rodata
	_landed_msg:
		.ascii "[ OK ] Landed in high memory\n\0"
	_kmain_msg:
		.ascii "[INFO] Starting kernel environment...\n\0"

.section .bss
	.align 16
	_stack_start:
		.skip 4096 // 4K stack
	_stack_end:

.section .text
	.type _start, @function
	_start:
		// Set the higher boot stack
		mov $_stack_end, %esp

		// We've landed!
		push $_landed_msg
		call _vga_print.boot
		add $4, %esp

		// Kmain boot text
		push $_kmain_msg
		call _vga_print.boot
		add $4, %esp

		// Pass Multiboot header to kmain
		mov (_mb_header.boot), %eax
		add VIRTUAL_OFFSET, %eax
		push %eax

		// Enter the kernel main
		call kmain

	// Hang the kernel if we ever get this far
	_hang:
		cli
		hlt
		jmp _hang
