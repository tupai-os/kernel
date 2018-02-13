// file : multiboot.s
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

.extern _start.boot

.set MB_MAGIC, 0xE85250D6
.set MB_ARCH,  0
.set MB_SIZE,  (mb_end - mb_start)
.set MB_CHECKSUM, (0 - (MB_MAGIC + MB_ARCH + MB_SIZE))

.section .rodata.multiboot
	.align 4
	mb_start:
			.align 4
		.long MB_MAGIC
		.long MB_ARCH
		.long MB_SIZE
		.long MB_CHECKSUM

		// Entry address tag
		.align 8
		.word 3            // Type
		.word 0            // Flags
		.long 12           // Size
		.long _start.boot // Entry address

		// Framebuffer tag
		//.align 8
		//.word 5   // Type
		//.word 0   // Flags
		//.long 20  // Size
		//.long 640 // Width
		//.long 480 // Height
		//.long 32  // BPP

		// End tag
		.align 8
		.word 0 // Type
		.word 0 // Flags
		.long 8 // Size
	mb_end:
