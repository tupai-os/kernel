@ file : start.s
@
@ Copyright (C) 2018  Joshua Barretto <joshua.s.barretto@gmail.com>
@
@ This program is free software: you can redistribute it and/or modify
@ it under the terms of the GNU General Public License as published by
@ the Free Software Foundation, either version 3 of the License, or
@ (at your option) any later version.
@
@ This program is distributed in the hope that it will be useful,
@ but WITHOUT ANY WARRANTY; without even the implied warranty of
@ MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
@ GNU General Public License for more details.
@
@ You should have received a copy of the GNU General Public License
@ along with this program.  If not, see <http://www.gnu.org/licenses/>.

.global _start.boot

@ Initial register values:
@ r0  -> 0x00000000
@ r1  -> 0x00000C42
@ r2  -> 0x00000100 = ATAGS start
@ r15 -> 0x00008000 = Execution start
.section .text.boot
	_start.boot:
		// Set up stack
		mov sp, #0x8000

		// Clear BSS
		ldr r4, =bss_start.boot
		ldr r9, =bss_end.boot
		mov r5, #0
		mov r6, #0
		mov r7, #0
		mov r8, #0
		b 2f
		1:
			stmia r4!, {r5 - r8}
		2:
			cmp r4, r9
			blo 1b

		ldr r3, =kmain
		blx r3

		_hang.boot:
			wfe
			b _hang.boot
