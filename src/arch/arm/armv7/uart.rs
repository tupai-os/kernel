// file : uart.rs
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

use arch::arm::mmio;
use arch::arm::mmio::{RegBlock, Reg32};
use arch::arm::board;

const GPIO_BASE: usize = 0x3F200000;

const GPPUD: usize = GPIO_BASE + 0x94;
const GPPUD_CLK0: usize = GPIO_BASE + 0x98;

#[allow(dead_code)]
#[repr(C)]
struct UartRegs {
	data              : Reg32, // 0
	irq_enable        : Reg32, // 4
	irq_identify      : Reg32, // 8
	_unused0          : Reg32, // 12
	modem_control     : Reg32, // 16
	raw_line_status   : Reg32, // 20
	masked_line_status: Reg32, // 24
	_unused1          : Reg32, // 28
	ilpr              : Reg32, // 32
	int_baud_div      : Reg32, // 36
	frac_baud_div     : Reg32, // 40
	line_control      : Reg32, // 44
	control           : Reg32, // 48
	unused2           : Reg32, // 52
	irq_mask          : Reg32, // 56
	unused4           : Reg32, // 60
	unused5           : Reg32, // 64
	irq_clear         : Reg32, // 68
}

lazy_static! {
	static ref UART: RegBlock<UartRegs> = RegBlock::new(board::UART0_BASE);
}

pub fn init() {
	// Disable UART0
	UART.lock().data.write(0);

	// Set up pins
	mmio::write32(GPPUD, 0);
	mmio::write32(GPPUD_CLK0, (1 << 14) | (1 << 15));
	mmio::write32(GPPUD_CLK0, 0);

	// Clear pending interrupts
	UART.lock().irq_clear.write(0x7FF);

	// Set baud rate
	UART.lock().int_baud_div.write(1);
	UART.lock().frac_baud_div.write(40);

	// Enable FIFO and 8-bit transmission (inc. 1 stop bit)
	UART.lock().line_control.write((1 << 4) | (1 << 5) | (1 << 6));

	// Mask all interrupts
	UART.lock().irq_mask.write(
		(1 << 1) |
		(1 << 4) |
		(1 << 5) |
		(1 << 6) |
		(1 << 7) |
		(1 << 8) |
		(1 << 9) |
		(1 << 10)
	);

	// Reenable UART0
	UART.lock().control.write((1 << 0) | (1 << 8) | (1 << 9));
}

pub fn write(data: u8) {
	while UART.lock().masked_line_status.read() & (1 << 5) != 0 {}
	UART.lock().data.write(data as u32)
}

pub fn read() -> u8 {
	while UART.lock().masked_line_status.read() & (1 << 4) != 0 {}
	UART.lock().data.read() as u8
}
