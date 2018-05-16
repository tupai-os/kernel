// file : pic.rs
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

use llapi::family::intrinsic::port::{out8, in8};

pub const REMAP_OFFSET: usize = 32;

const PORT_PIC1_CMD: u16 = 0x20;
const PORT_PIC2_CMD: u16 = 0xA0;

const PORT_PIC1_DATA: u16 = PORT_PIC1_CMD + 1;
const PORT_PIC2_DATA: u16 = PORT_PIC2_CMD + 1;

const EOI: u8 = 0x20;

#[allow(dead_code)]
#[repr(u8)]
enum ICW1 {
	Init     = 1 << 4,
	ICW4     = 1 << 0,
	Single   = 1 << 1,
	Interval = 1 << 2,
	Level    = 1 << 3,
}

#[allow(dead_code)]
#[repr(u8)]
enum ICW4 {
	Mode8086       = 1 << 0,
	Auto           = 1 << 1,
	BufferedSlave  = 1 << 3,
	BufferedMaster = 3 << 3,
	Nested         = 1 << 4,
}

pub fn init() {
	// Start initiation
	out8(PORT_PIC1_CMD, ICW1::Init as u8 | ICW1::ICW4 as u8);
	out8(PORT_PIC2_CMD, ICW1::Init as u8 | ICW1::ICW4 as u8);

	// Now, Pass the PICs 3 initiation bytes

	// Offset
	out8(PORT_PIC1_DATA, REMAP_OFFSET as u8);
	out8(PORT_PIC2_DATA, REMAP_OFFSET as u8 + 8);

	// Cascade identity
	out8(PORT_PIC1_DATA, 4);
	out8(PORT_PIC2_DATA, 2);

	// Operate in 8086 mode
	out8(PORT_PIC1_DATA, ICW4::Mode8086 as u8);
	out8(PORT_PIC2_DATA, ICW4::Mode8086 as u8);

	// Mask all interrupts
	out8(PORT_PIC1_DATA, 0xFF);
	out8(PORT_PIC2_DATA, 0xFF);

	// TODO: Fix this
	logok!("Initiated and remapped PIC");
}

pub fn unmask(irq: usize) {
	if irq < 16 {
		let port: u16 = if irq < 8 { PORT_PIC1_DATA } else { PORT_PIC2_DATA };
		let mask = in8(port) & !(1 << (irq as u8 & 0b111));
		out8(port, mask);
	}
}

pub fn eoi(irq: usize) {
	if irq < 16 {
		if irq >= 8 {
			out8(PORT_PIC2_CMD, EOI);
		}
		out8(PORT_PIC1_CMD, EOI);
	}
}
