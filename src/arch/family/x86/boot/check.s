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
	_cpuid_msg_good:
		.ascii "[ OK ] CPUID is supported\n\0"
	_cpuid_msg_bad:
		.ascii "[FAIL] Kernel requires CPUID support\n\0"
	_paging_msg_good:
		.ascii "[ OK ] Paging is supported\n\0"
	_paging_msg_bad:
		.ascii "[FAIL] Kernel requires paging support\n\0"
	_checks_good:
		.ascii "[ OK ] Initial checks passes successfully\n\0"

.code32
.section .text.boot
	_check.boot:
		push %ebp
		mov %esp, %ebp

		// CHECK MULTIBOOT MAGIC

		cmpl $MB_VALID_MAGIC, (_mb_magic.boot)
		je 2f
		1:
			// Multiboot magic failure
			push $_mb_msg_bad
			call _vga_print.boot
			add $4, %esp

			jmp _hang.boot
		2:
			// Multiboot magic success
			push $_mb_msg_good
			call _vga_print.boot
			add $4, %esp

		// CHECK CPUID SUPPORTED

		// Read eflags and place into eax and ecx
		pushf
		pop %eax
		mov %eax, %ecx

		// Flip the cpuid bit and write it back into eflags
		xor $(1 << 21), %eax
		push %eax
		popf

		// Read eflags again
		pushf
		pop %eax

		// Return eflags to original value in ecx
		push %ecx
		popf

		// Has eflags changed?
		cmp %eax, %ecx
		jne 2f
		1:
			// CPUID is unsupported
			push $_cpuid_msg_bad
			call _vga_print.boot
			add $4, %esp

			jmp _hang.boot
		2:
			// CPUID is supported
			push $_cpuid_msg_good
			call _vga_print.boot
			add $4, %esp

		// CHECK PAGING SUPPORTED

		// Calling CPUID with eax = 1 will give us features
		mov $1, %eax
		cpuid

		// Is the paging flag set?
		and $1, %edx
		cmp $1, %edx
		je 2f
		1:
			// Paging is unsupported
			push $_paging_msg_bad
			call _vga_print.boot
			add $4, %esp

			jmp _hang.boot
		2:
			// Paging is supported
			push $_paging_msg_good
			call _vga_print.boot
			add $4, %esp

		// FINISH CHECKS

		push $_checks_good
		call _vga_print.boot
		add $4, %esp

		pop %ebp
		ret
