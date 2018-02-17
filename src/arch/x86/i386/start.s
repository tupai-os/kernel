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

.extern _vga_print.boot
.extern _check.boot
.extern _paging_init.boot
.extern _paging_enable.boot
.extern kmain

.global _start.boot
.global _hang.boot

.set VIRTUAL_OFFSET, 0xC0000000

.section .rodata.boot
	_boot_msg:
		.ascii "[INFO] Starting early kernel...\n\0"
	_highjump_msg:
		.ascii "[INFO] Jumping to high memory...\n\0"

.section .bss.boot
	.align 16
	_stack_start.boot:
		.skip 256 // 256B stack
	_stack_end.boot:

	_mb_magic.boot:
		.long
	_mb_header.boot:
		.long

.section .text.boot
	.type _start.boot, @function
	_start.boot:
		// Set the initial boot stack
		mov $_stack_end.boot, %esp

		// Preserve Multiboot attributes
		mov %ebx, (_mb_magic.boot)
		mov %eax, (_mb_header.boot)

		// Initial boot text
		push $_boot_msg
		call _vga_print.boot
		add $4, %esp

		// Ensure we're running on a supported system
		call _check.boot

		// Setup higher-half kernel paging
		call _paging_init.boot
		call _paging_enable.boot

		// Kmain boot text
		push $_highjump_msg
		call _vga_print.boot
		add $4, %esp

		// Jump to higher memory
		call _start

	// Hang the kernel if we ever get this far
	_hang.boot:
		cli
		hlt
		jmp _hang.boot

// HIGHER HALF

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
