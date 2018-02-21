// file : paging.s
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

.global _paging_init.boot
.global _paging_enable.boot

.set PAGES, 1024 * 128 // 512M of memory

.section .bss.boot
	.align 4096
	_p2_table:
		.skip 4 * 1024
	_p1_table:
		.skip 4 * PAGES // 4K * 1024 * 128 = 512M of memory
	_tables_end:

.section .rodata.boot
	_paging_init_msg:
		.ascii "[ OK ] Trampoline page tables created\n\0"
	_paging_enabled_msg:
		.ascii "[ OK ] Paging enabled, kernel mapped\n\0"

.code32
.section .text.boot
	_paging_init.boot:
		push %ebp
		mov %esp, %ebp

		// Clear page tables first
		mov $_p2_table, %ecx
		1:
			movl $0, (%ecx)
			add $4, %ecx
			cmp $_tables_end, %ecx
			jne 1b

		// Map P2 table entries
		mov $0, %ecx
		1:
			// Map the ECX-th P1 entry to a page at address 4K * ecx
			mov $(1024 * 4), %eax // For each P1 table (size = 1024 * 4)
			imul %ecx, %eax // Calculate 4K * ecx
			or $0b11, %eax // Present and writable
			add $_p1_table, %eax // Present, writable

			// Lower identity
			mov $4, %edx
			imul %ecx, %edx // Calculate (p2_table + ecx * 8)...
			add $_p2_table, %edx
			mov %eax, (%edx) // Map ECX-th entry (each entry is 8 bytes)...

			// Higher virtual
			mov $4, %edx
			imul %ecx, %edx // Calculate (p2_table + ecx * 8)...
			add $_p2_table, %edx
			add $((1024 - 256) * 4), %edx // 3G offset
			mov %eax, (%edx) // Map ECX-th entry (each entry is 8 bytes)...

			// Iterate the loop
			inc %ecx
			cmp $128, %ecx
			jne 1b

		// Map P1 table entries
		mov $0, %ecx
		2:
			// Map the ECX-th P1 entry to a page at address 4K * ecx
			mov $4096, %eax // 4K
			imul %ecx, %eax // Calculate 4K * ecx
			or $0b11, %eax // Present, writable

			mov $4, %edx
			imul %ecx, %edx // Calculate (p1_table + ecx * 4)...
			add $_p1_table, %edx
			mov %eax, (%edx) // Map ECX-th entry (each entry is 4 bytes)...

			// Iterate the loop
			inc %ecx
			cmp $PAGES, %ecx
			jne 2b

		push $_paging_init_msg
		call _vga_print.boot
		add $4, %esp

		pop %ebp
		ret

	_paging_enable.boot:
		push %ebp // Save the state of EBP
		mov %esp, %ebp // Save the state of ESP in EBP

		mov $_p2_table, %eax // Find the function argument on the stack
		mov %eax, %cr3 // Move it into the CR3 register

		mov %ebp, %esp // Restore the state of ESP from EBP
		pop %ebp // Restore the state of EBP

		push %ebp // Save the state of EBP
		mov %esp, %ebp // Save the state of ESP in EBP

		mov %cr0, %eax // Place the value of the CR0 register in EAX
		or (1 << 31), %eax // Enable the paging bit
		mov %eax, %cr0 // Place the new CR0 value back into CR0

		mov %ebp, %esp // Restore the state of ESP from EBP
		pop %ebp // Restore the state of EBP

		push $_paging_enabled_msg
		call _vga_print.boot
		add $4, %esp

		ret
