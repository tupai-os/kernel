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

.extern _vga_print64.boot

.global _paging_init64.boot
.global _paging_enable64.boot

.set PAGES, 512 * 256 // 512M of memory

.section .bss.boot
	// Page tables 'n stuff
	.align 4096
	_p4_table64:
		.skip 512 * 8
	_p3_table64:
		.skip 512 * 8
	_p2_table64:
		.skip 512 * 8
	_p1_tables64:
		.skip 8 * PAGES // Enough to cover 4K * 512 * 256 = 512M of memory
	_tables_end64:

.section .rodata.boot
	_paging_init_msg64:
		.ascii "[ OK ] Trampoline page tables created\n\0"
	_paging_enabled_msg64:
		.ascii "[ OK ] Higher page mapping enabled\n\0"

.code64
.section .text.boot
	_paging_init64.boot:
		push %rbp
		mov %rsp, %rbp

		//Clear page tables first
		mov $_p4_table64, %rcx
		1:
			movl $0, (%rcx)
			add $4, %rcx
			cmp $_tables_end64, %rcx
			jne 1b

		// IDENTITY MAPPING

		// Map 1st P4 entry to P3 table
		mov $_p3_table64, %rax
		or $0b11, %rax // Present and writable
		mov %rax, (_p4_table64)

		// Map 1st P3 entry to P2 table
		mov $_p2_table64, %rax
		or $0b11, %rax // Present and writable
		mov %rax, (_p3_table64)

		// HIGHER HALF MAPPING

		// Map 256th P4 entry to P3 table (virtual higher mapping)
		mov $_p3_table64, %rax
		or $0b11, %rax // Present and writable
		mov %rax, (_p4_table64 + 511 * 8)

		// Map 511th P3 entry to P2 table (virtual higher mapping)
		mov $_p2_table64, %rax
		or $0b11, %rax // Present and writable
		mov %rax, (_p3_table64 + 510 * 8)

		// Map P2 table entries
		mov $0, %rcx
		1:
			// Map the rcx-th P1 entry to a page at address 4K * rcx
			mov $(512 * 8), %rax    // For each P1 table (size = 512 * 8)
			imul %rcx, %rax         // Calculate 4K * rcx
			or $0b11, %rax          // Present and writable
			add $_p1_tables64, %rax // Present, writable

			mov $8, %rdx
			imul %rcx, %rdx     // Calculate (p2_table + rcx * 8)...
			add $_p2_table64, %rdx

			mov %rax, (%rdx)    // Map rcx-th entry (each entry is 8 bytes)...

			// Iterate the loop
			inc %rcx
			cmp $(PAGES >> 9), %rcx
			jne 1b

		// Map P1 table entries
		mov $0, %rcx
		2:
			// Map the rcx-th P1 entry to a page at address 4K * rcx
			mov $4096, %rax     // 4K
			imul %rcx, %rax     // Calculate 4K * rcx
			or $0b11, %rax      // Present, writable

			mov $8, %rdx
			imul %rcx, %rdx     // Calculate (p1_table + rcx * 8)...
			add $_p1_tables64, %rdx

			mov %rax, (%rdx)    // Map rcx-th entry (each entry is 8 bytes)...

			// Iterate the loop
			inc %rcx
			cmp $(PAGES), %rcx
			jne 2b

		mov $_paging_init_msg64, %rdi
		call _vga_print64.boot

		pop %rbp
		retq

	// Enable paging
	_paging_enable64.boot:
		push %rbp
		mov %rsp, %rbp

		// Load P4 table to CR3 (so the CPU knows where to look for paging info)
		mov $_p4_table64, %rax
		mov %rax, %cr3

		mov $_paging_enabled_msg64, %rdi
		call _vga_print64.boot

		// We've enabled 64-bit paging

		pop %rbp
		retq
