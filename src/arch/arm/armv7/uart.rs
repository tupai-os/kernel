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

const GPIO_BASE: usize = 0x3F200000;

const GPPUD: usize = GPIO_BASE + 0x94;
const GPPUD_CLK0: usize = GPIO_BASE + 0x98;

const UART0_BASE: usize = 0x3F201000;
const UART0_DR: usize = UART0_BASE + 0x0;
const UART0_FR: usize = UART0_BASE + 0x4;
const UART0_IBRD: usize = UART0_BASE + 0x24;
const UART0_FBRD: usize = UART0_BASE + 0x28;
const UART0_LCRH: usize = UART0_BASE + 0x2C;
const UART0_CR: usize = UART0_BASE + 0x30;
const UART0_IMSC: usize = UART0_BASE + 0x38;
const UART0_ICR: usize = UART0_BASE + 0x44;

pub fn init() {
	// Disable UART0
	mmio::write32(UART0_DR, 0);

	// Set up pins
	mmio::write32(GPPUD, 0);
	mmio::write32(GPPUD_CLK0, (1 << 14) | (1 << 15));
	mmio::write32(GPPUD_CLK0, 0);

	// Clear pending interrupts
	mmio::write32(UART0_ICR, 0x7FF);

	// Set baud rate
	mmio::write32(UART0_IBRD, 1);
	mmio::write32(UART0_FBRD, 40);

	// Enable FIFO and 8-bit transmission (inc. 1 stop bit)
	mmio::write32(UART0_LCRH, (1 << 4) | (1 << 5) | (1 << 6));

	// Mask all interrupts
	mmio::write32(UART0_IMSC,
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
	mmio::write32(UART0_CR, (1 << 0) | (1 << 8) | (1 << 9));
}

pub fn write(data: u8) {
	while (mmio::read32(UART0_FR) & (1 << 5)) > 0 {}
	mmio::write32(UART0_DR, data as u32);
}

pub fn read() -> u8 {
	while (mmio::read32(UART0_FR) & (1 << 4)) > 0 {}
	mmio::read32(UART0_DR) as u8
}
