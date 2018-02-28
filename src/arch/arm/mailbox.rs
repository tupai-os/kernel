// file : bcm2836.rs
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

use arch::arm::mmio::{RegBlock, Reg32};
use super::board;

#[allow(dead_code)]
#[repr(u8)]
enum Channel {
	PowerManagement = 0,
	Framebuffer     = 1,
	VirtualUart     = 2,
	VideoCore       = 3,
	Leds            = 4,
	Buttons         = 5,
	Touchscreen     = 6,
	_unused         = 7,
	TagsArmToVc     = 8,
	TagsVcToArm     = 9,
}

#[allow(dead_code)]
#[repr(C)]
struct Mailbox {
	read   : Reg32,
	_unused: [Reg32; 3],
	peek   : Reg32,
	sender : Reg32,
	status : Reg32,
	config : Reg32,
	write  : Reg32,
}

const MAILBOX_FULL: u32 = 1 << 31;
const MAILBOX_EMPTY: u32 = 1 << 30;

lazy_static! {
	static ref MAILBOX: RegBlock<Mailbox> = RegBlock::new(board::MAILBOX_BASE);
}

pub fn write(channel: Channel, addr: u32) {
	if addr & 0xF != 0 {
		panic!("Mailbox address must be 16-aligned");
	} else {
		// Acquire mailbox lock
		let mut mailbox = MAILBOX.lock();

		// Wait for mailbox to empty, then write
		while mailbox.status.read() & MAILBOX_FULL != 0 {}
		mailbox.write.write(addr | channel as u32);
	}
}

pub fn read(channel: Channel) -> bool {
	// Acquire mailbox lock
	let mut mailbox = MAILBOX.lock();

	// Wait for mailbox to empty, then write
	while mailbox.status.read() & MAILBOX_EMPTY != 0 {}
	mailbox.write.write(addr | channel as u32);

	// Read if the channel matches
	if mailbox.read.read() & 0xF == channel as u32 {
		return mailbox.read.read() >> 4 == 0;
	} else {
		return false;
	}
}
