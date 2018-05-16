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

.set PAGES, 512 * 256 // 512M of memory

.section .bss.boot
	.align 4096
	_p4_table:
		.skip 8 * 512
	_p3_lo_table:
		.skip 8 * 512
	_p2_lo_table:
		.skip 8 * 512
	_p1_lo_tables:
		.skip 8 * PAGES // Enough to cover all pages
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
		mov $_p4_table, %ecx
		1:
			movl $0, (%ecx)
			add $4, %ecx
			cmp $_tables_end, %ecx
			jne 1b

		// Map 1st P4 entry to P3 table
		mov $_p3_lo_table, %eax
		or $0b11, %eax // Present and writable
		mov %eax, (_p4_table)

		// Map 1st P3 entry to P2 table
		mov $_p2_lo_table, %eax
		or $0b11, %eax // Present and writable
		mov %eax, (_p3_lo_table)

		// Map P2 table entries
		mov $0, %ecx
		1:
			// Map the ECX-th P1 entry to a page at address 4K * ecx
			mov $(512 * 8), %eax    // For each P1 table (size = 512 * 8)
			imul %ecx, %eax         // Calculate 4K * ecx
			or $0b11, %eax          // Present and writable
			add $_p1_lo_tables, %eax // Present, writable

			mov $8, %edx
			imul %ecx, %edx     // Calculate (p2_table + ecx * 8)...
			add $_p2_lo_table, %edx

			mov %eax, (%edx)    // Map ECX-th entry (each entry is 8 bytes)...

			// Iterate the loop
			inc %ecx
			cmp $256, %ecx
			jne 1b

		// Map P1 table entries
		mov $0, %ecx
		2:
			// Map the ECX-th P1 entry to a page at address 4K * ecx
			mov $4096, %eax     // 4K
			imul %ecx, %eax     // Calculate 4K * ecx
			or $0b11, %eax      // Present, writable

			mov $8, %edx
			imul %ecx, %edx     // Calculate (p1_table + ecx * 8)...
			add $_p1_lo_tables, %edx

			mov %eax, (%edx)    // Map ECX-th entry (each entry is 8 bytes)...

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
		push %ebp
		mov %esp, %ebp

		// Load P4 table to CR3 (so the CPU knows where to look for paging info)
		mov $_p4_table, %eax
		mov %eax, %cr3

		// Enable PAE flag in CR4 (Physical Address Extension)
		mov %cr4, %eax
		or $(1 << 5), %eax // (6th bit)
		mov %eax, %cr4

		// Set the long-mode bit in the EFER MSR (Model-Specific Register)
		mov $0xC0000080, %ecx
		rdmsr // Load MSR into EAX
		or $(1 << 8), %eax // (9th bit)
		wrmsr // Load EAX back into MSR

		// Enable paging in CR0
		mov %cr0, %eax
		or $(1 << 31), %eax // (32nd bit)
		mov %eax, %cr0

		// We've enabled paging (we're still in a 32-bit compatibility mode though)

		pop %ebp
		ret
