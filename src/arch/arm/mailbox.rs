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
use arch::base::isa;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Channel {
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

pub fn recv(channel: Channel) -> Result<u32, ()> {
	if channel as u8 > 0xF {
		logln!("Mailbox channel cannot be larger than 16");
		Err(())
	} else {
		let mut mailbox = MAILBOX.lock();

		loop {
			while mailbox.status.read() & MAILBOX_EMPTY != 0 {
				isa::flush_cache();
			}

			isa::mem_barrier();
			let msg = mailbox.read.read();
			isa::mem_barrier();

			if msg & 0xF == channel as u32 {
				return Ok(msg & !0xF)
			}
		}
	}
}

pub fn send(channel: Channel, data: u32) -> Result<(), ()> {
	if data & 0xF != 0 {
		logln!("Mailbox data address must be 16-aligned");
		Err(())
	} else {
		// Acquire mailbox lock
		let mut mailbox = MAILBOX.lock();

		// Wait for mailbox to empty, then write
		while mailbox.status.read() & MAILBOX_FULL != 0 {
			logln!("Waiting for mailbox to be empty...");
			isa::flush_cache();
		}

		isa::mem_barrier();
		mailbox.write.write(data | channel as u32);

		Ok(())
	}
}
