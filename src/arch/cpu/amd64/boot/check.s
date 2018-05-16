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
.extern _hang.boot

.global _check_long.boot

.section .rodata.boot
	_long_msg_good:
		.ascii "[ OK ] Long mode is supported\n\0"
	_long_msg_bad:
		.ascii "[FAIL] Kernel requires long mode\n\0"

.code32
.section .text.boot
	_check_long.boot: // Note: This assumes all checks in _check.boot have passed
		push %ebp
		mov %esp, %ebp

		// Calling CPUID with eax = 1 will give us features
		mov $0x80000000, %eax
		cpuid

		// Is the extended function supported?
		cmp $0x80000001, %eax
		jne 2f

		// Is the long mode flag set?
		mov $0x80000001, %eax
		cpuid
		test (1 << 29), %edx
		jnz 2f
		1:
			// Long mode is unsupported
			push $_long_msg_bad
			call _vga_print.boot
			add $4, %esp

			jmp _hang.boot
		2:
			// Long mode is supported
			push $_long_msg_good
			call _vga_print.boot
			add $4, %esp

		pop %ebp
		ret
