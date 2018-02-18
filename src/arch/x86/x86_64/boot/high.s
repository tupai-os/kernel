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

.extern _vga_print64.boot
.extern kmain

.global _start_high64
.global _hang64

.set VIRTUAL_OFFSET, 0xFFFFFFFF80000000

.section .rodata
	_landed_msg:
		.ascii "[ OK ] Landed in high memory\n\0"
	_kmain_msg:
		.ascii "[INFO] Starting kernel environment...\n\0"

.section .bss
	.align 16
	_stack_start:
		.skip 8192 // 8K stack
	_stack_end:

.code64
.section .text
	.type _start_high64, @function
	_start_high64:
		// Set the higher boot stack
		mov $_stack_end, %rsp

		// We've landed!
		mov $_landed_msg, %rdi
		call _vga_print64.boot

		// Kmain boot text
		mov $_kmain_msg, %rdi
		call _vga_print64.boot

		// Pass Multiboot header to kmain
		mov (_mb_header.boot), %rdi
		add $VIRTUAL_OFFSET, %rdi

		// Enter the kernel main
		call kmain

	// Hang the kernel if we ever get this far
	_hang_high64:
		cli
		hlt
		jmp _hang_high64
