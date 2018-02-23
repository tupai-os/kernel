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

	.align 4
	_exception_handler0: // DivZero Exception
		push $EXCEPTION_DUMMY_ERROR // Dummy error
		PUSH_REGS
		push %esp // Pass stack frame
		call divzero_handler
		POP_REGS
		iret

	.align 4
	_exception_handler1: // Debug Exception
		push $EXCEPTION_DUMMY_ERROR // Dummy error
		PUSH_REGS
		push %esp // Pass stack frame
		call debug_handler
		POP_REGS
		iret
