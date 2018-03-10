// file : exception.s
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

.global _exception_handler0
.global _exception_handler1
.global _exception_handler2
.global _exception_handler3
.global _exception_handler4
.global _exception_handler5
.global _exception_handler6
.global _exception_handler7
.global _exception_handler8
.global _exception_handler9
.global _exception_handler10
.global _exception_handler11
.global _exception_handler12
.global _exception_handler13
.global _exception_handler14
// <Reserved>
.global _exception_handler16
.global _exception_handler17
.global _exception_handler18
.global _exception_handler19
.global _exception_handler20
// <Reserved>
.global _exception_handler30

.extern div_zero_handler

.set EXCEPTION_DUMMY_ERROR, 0

.section .text
	.macro PUSH_REGS
		push %eax
		push %ebx
		push %ecx
		push %edx
		push %esi
		push %edi
		push %ebp
		cld
	.endm

	.macro POP_REGS
		pop %ebp
		pop %edi
		pop %esi
		pop %edx
		pop %ecx
		pop %ebx
		pop %eax
	.endm

	.macro ERROR_EXCEPTION n, name
		.align 4
		_exception_handler\n\():
			push $\n\() // Push exception ID
			PUSH_REGS
			push %esp // Pass stack frame
			call \name\()_handler
			POP_REGS
			iret
	.endm

	.macro DUMMY_EXCEPTION n, name
		.align 4
		_exception_handler\n\():
			push $EXCEPTION_DUMMY_ERROR // Dummy error
			push $\n\() // Push exception ID
			PUSH_REGS
			push %esp // Pass stack frame
			call \name\()_handler
			POP_REGS
			iret
	.endm

	DUMMY_EXCEPTION 0 divzero
	DUMMY_EXCEPTION 1 debug
	DUMMY_EXCEPTION 2 unimplemented
	DUMMY_EXCEPTION 3 unimplemented
	DUMMY_EXCEPTION 4 unimplemented
	DUMMY_EXCEPTION 5 unimplemented
	DUMMY_EXCEPTION 6 unimplemented
	DUMMY_EXCEPTION 7 unimplemented
	ERROR_EXCEPTION 8 unimplemented
	DUMMY_EXCEPTION 9 unimplemented
	ERROR_EXCEPTION 10 unimplemented
	ERROR_EXCEPTION 11 segnotpresent
	ERROR_EXCEPTION 12 unimplemented
	ERROR_EXCEPTION 13 gprotectionfault
	ERROR_EXCEPTION 14 pagefault
	DUMMY_EXCEPTION 16 unimplemented
	ERROR_EXCEPTION 17 unimplemented
	DUMMY_EXCEPTION 18 unimplemented
	DUMMY_EXCEPTION 19 unimplemented
	DUMMY_EXCEPTION 20 unimplemented
	ERROR_EXCEPTION 30 unimplemented
