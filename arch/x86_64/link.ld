/*
// file : link.ld
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
*/

ENTRY(_start.boot)
OUTPUT_FORMAT(elf64-x86-64)

SECTIONS
{
	. = 1M;
	KERNEL_VIRTUAL = 0xFFFFFFFF80000000;
	KHEAP_SIZE = 4M;

	kernel_start = .;

	/* Place the Multiboot header at the beginning */
	.rodata.multiboot : ALIGN(4K)
	{
		KEEP(*(.rodata.multiboot))
	}

	.text.boot : ALIGN(4K)
	{
		*(.text.boot)
	}

	.data.boot : ALIGN(4K)
	{
		*(.rodata.boot)
		*(.data.boot)
	}

	.bss.boot : ALIGN(4K)
	{
		*(.bss.boot)
	}

	. = . + KERNEL_VIRTUAL;

	.text ALIGN(4K) : AT(ADDR(.text) - KERNEL_VIRTUAL)
	{
		*(.init*)
		*(.fini*)

		*(.text)
		*(.gnu.linkonce.t*)
	}

	.rodata ALIGN(4K) : AT(ADDR(.rodata) - KERNEL_VIRTUAL)
	{
		*(.rodata)

		start_ctors = .;
		*(SORT(.ctors*))
		end_ctors = .;

		start_dtors = .;
		*(SORT(.dtors*))
		end_dtors = .;

		*(.gnu.linkonce.r*)
	}

	.data ALIGN(4K) : AT(ADDR(.data) - KERNEL_VIRTUAL)
	{
		*(.data)
		*(.gnu.linkonce.d*)
	}

	.bss ALIGN(4K) : AT(ADDR(.bss) - KERNEL_VIRTUAL)
	{
		*(COMMON)
		*(.bss)
		*(.gnu.linkonce.b*)
	}

	.bss.noload ALIGN(4K) (NOLOAD) : AT(ADDR(.bss.noload) - KERNEL_VIRTUAL)
	{
		*(.bss.noload)

		. = ALIGN(4K);
		kheap_start = .;
		. += KHEAP_SIZE;
		kheap_end = .;
	}

	. = . - KERNEL_VIRTUAL;
	kernel_end = .;
}
