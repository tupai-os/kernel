@ file : exception.s
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

.global _exception_table_start
.global _exception_table_end
.global _relocate_exception_table

.section .text
	_exception_table_start:
		ldr pc, _ptr_reset_handler
		ldr pc, _ptr_invalidop_handler
		ldr pc, _ptr_swi_handler
		ldr pc, _ptr_prefetchabort_handler
		ldr pc, _ptr_dataabort_handler
		ldr pc, _ptr_reset_handler
		ldr pc, _ptr_hwi_handler
		ldr pc, _ptr_firq_handler

		_ptr_reset_handler:
			.word _reset_handler
		_ptr_invalidop_handler:
			.word _invalidop_handler
		_ptr_swi_handler:
			.word _swi_handler
		_ptr_prefetchabort_handler:
			.word _prefetchabort_handler
		_ptr_dataabort_handler:
			.word _dataabort_handler
		_ptr_hwi_handler:
			.word _hwi_handler
		_ptr_firq_handler:
			.word _firq_handler
	_exception_table_end:

.section .text
	@ Relocate the exception table
	_relocate_exception_table:
		push  {r4-r9} @ Preserve registers

		@ Load in src and dest addresses
		ldr   r0, =_exception_table_start
		mov   r1, #0x0000

		@ Load & store registers multiple times to move table
		ldmia r0!, {r2-r9}
		stmia r1!, {r2-r9}
		ldmia r0!, {r2-r8}
		stmia r1!, {r2-r8}

		pop   {r4-r9} @ Restore registers
		blx   lr

	.macro IRQ_PREFIX
		srsdb sp!, #0x13       @ Store the cpsr and sp on the supervisor stack
		cpsid if, #0x13        @ Switch to supervisor mode, IRQs disabled

		push  {r0-r3, r12, lr} @ Preserve CPU context

		mov   r0, sp           @ Pass stack frame argument

		and   r1, sp, #7       @ Deal with stack misalignment
		sub   sp, sp, r1

		push  {r1}
	.endm

	.macro IRQ_SUFFIX
		pop   {r1}

		add   sp, sp, r1       @ Restore stack misalignment

		pop   {r0-r3, r12, lr} @ Restore CPU context
		rfeia sp!
	.endm

	@ Software interrupt handler
	_swi_handler:
		IRQ_PREFIX
		bl    swi_handler      @ Branch to SWI handler
		IRQ_SUFFIX

	@ Hardware interrupt handler
	_hwi_handler:
		sub   lr, lr, #4       @ Skip lr back to interrupted instruction
		IRQ_PREFIX
		bl    hwi_handler      @ Branch to HWI handler
		IRQ_SUFFIX

	@ Unimplemented handler
	_unimplemented_handler:
		sub   lr, lr, #4       @ Skip lr back to interrupted instruction
		IRQ_PREFIX
		bl    unimplemented_handler    @ Branch to unimplemented handler
		IRQ_SUFFIX

	@ Stubs for the rest
	1:
		_reset_handler:
		_invalidop_handler:
		_prefetchabort_handler:
		_dataabort_handler:
		_firq_handler:
			b _unimplemented_handler
