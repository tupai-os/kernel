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
.extern _vga_print64.boot
.extern _check.boot
.extern _check_long.boot
.extern _paging_init.boot
.extern _paging_enable.boot
.extern _paging_init64.boot
.extern _paging_enable64.boot
.extern _start_high64

.global _start.boot
.global _hang.boot

.set VIRTUAL_OFFSET, 0xC0000000

.section .rodata.boot
	_boot_msg:
		.ascii "[INFO] Starting early kernel...\n\0"
	_long_msg:
		.ascii "[INFO] Switching to long mode...\n\0"
	_long_good_msg:
		.ascii "[ OK ] Long mode entered successfully\n\0"
	_highjump_msg:
		.ascii "[INFO] Jumping to high memory...\n\0"

	_gdt64_start:
		.quad 0 // null segment
		.quad 0x0020980000000000 // Code segment
	_gdt64_end:
	_gdt64_ptr:
		.word (_gdt64_end - _gdt64_start - 1) // Limit
		.quad _gdt64_start

.section .bss.boot
	.align 16
	_stack_start.boot:
		.skip 256 // 256B stack
	_stack_end.boot:

	_mb_magic.boot:
		.long 0
	_mb_header.boot:
		.long 0

.code32
.section .text.boot
	.type _start.boot, @function
	_start.boot:
		// Set the initial boot stack
		mov $_stack_end.boot, %esp

		// Preserve Multiboot attributes
		mov %eax, (_mb_magic.boot)
		mov %ebx, (_mb_header.boot)

		// Initial boot text
		push $_boot_msg
		call _vga_print.boot
		add $4, %esp

		// Ensure we're running on a supported system
		call _check.boot
		call _check_long.boot

		// Setup higher-half kernel paging
		call _paging_init.boot
		call _paging_enable.boot

		// Long mode text
		push $_long_msg
		call _vga_print.boot
		add $4, %esp

		// Load 64-bit GDT
		lgdt (_gdt64_ptr)

		// Perform a non-local jump to the 64-bit entry
		// Reload the code seg register, switching the CPU to long mode
		jmp $8, $_start64.boot

	// Hang the kernel if we ever get this far
	_hang.boot:
		cli
		hlt
		jmp _hang.boot

.code64
.section .text.boot
	_start64.boot:

		// Set data selectors to null segment (x86_64 doesn't need them)
		mov $0, %ax
		mov %ax, %ss
		mov %ax, %ds
		mov %ax, %es
		mov %ax, %fs
		mov %ax, %gs

		// Long mode worked
		mov $_long_good_msg, %rdi
		call _vga_print64.boot

		// Enable long mode paging
		call _paging_init64.boot
		call _paging_enable64.boot

		// High jump message
		mov $_highjump_msg, %rdi
		call _vga_print64.boot

		// Jump to high memory
		movabs $_start_high64, %rax
		jmp *%rax

	// Hang the kernel if we ever get this far
	_hang64.boot:
		cli
		hlt
		jmp _hang64.boot
