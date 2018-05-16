// file : bcm2835.rs
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

use arch::arm::bcm2835;

use core::fmt;
use spin::Mutex;

const GPU_CACHE: u32 = 0x40000000;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
struct VideoCoreBuffer<T> {
	buffer_size: u32,
	code: u32,
	tags: T,
	padding: [u32; 3],
}

#[repr(packed, C)]
#[derive(Debug)]
struct VideoCoreTag<T> {
	identity: u32,
	buffer_size: u32,
	code: u32,
	data: T,
}

#[repr(packed, C)]
#[derive(Debug)]
struct SizeData {
	width: u32,
	height: u32,
}

#[repr(packed, C)]
#[derive(Debug)]
struct VirtSizeData {
	virt_width: u32,
	virt_height: u32,
}

#[repr(packed, C)]
#[derive(Debug)]
struct DepthData {
	depth: u32,
}

#[repr(packed, C)]
#[derive(Debug)]
struct ModeTags {
	size_tag: VideoCoreTag<SizeData>,
	virt_size_tag: VideoCoreTag<VirtSizeData>,
	depth_tag: VideoCoreTag<DepthData>,
	null_tag: u32,
}

#[repr(packed, C)]
#[derive(Debug)]
struct FbTags {
	fb_tag: VideoCoreTag<[u32; 2]>,
	null_tag: u32,
}

impl <T> VideoCoreBuffer<T> {
	const fn new(tags: T) -> VideoCoreBuffer<T> {
		use core::mem::size_of;
		VideoCoreBuffer::<T> {
			buffer_size: size_of::<VideoCoreBuffer<T>>() as u32,
			code: 0,
			tags: tags,
			padding: [0; 3],
		}
	}
}

impl <T> VideoCoreTag<T> {
	const fn new(identity: u32, data: T) -> VideoCoreTag<T> {
		use core::mem::size_of;
		VideoCoreTag::<T> {
			identity: identity,
			buffer_size: size_of::<T>() as u32,
			code: 0,
			data: data,
		}
	}
}

impl ModeTags {
	const fn new() -> ModeTags {
		ModeTags {
			size_tag: VideoCoreTag::new(0x00048003, SizeData {
				width: WIDTH,
				height: HEIGHT,
			}),
			virt_size_tag: VideoCoreTag::new(0x00048004, VirtSizeData {
				virt_width: WIDTH,
				virt_height: HEIGHT,
			}),
			depth_tag: VideoCoreTag::new(0x00048005, DepthData {
				depth: 32,
			}),
			null_tag: 0,
		}
	}
}

impl FbTags {
	const fn new() -> FbTags {
		FbTags {
			fb_tag: VideoCoreTag::new(0x00040001, [16, 0]),
			null_tag: 0,
		}
	}
}

static mut VC_MODE_BUFFER: VideoCoreBuffer<ModeTags> = VideoCoreBuffer::new(ModeTags::new());
static mut VC_FB_BUFFER: VideoCoreBuffer<FbTags> = VideoCoreBuffer::new(FbTags::new());

pub fn init() {
	use arch::arm::mailbox;

	// Set screen characteristics
	mailbox::send(mailbox::Channel::TagsArmToVc, unsafe { &VC_MODE_BUFFER as *const _ as u32 });
	// Request framebuffer
	mailbox::send(mailbox::Channel::TagsArmToVc, unsafe { &VC_FB_BUFFER as *const _ as u32 });
	let data = mailbox::recv(mailbox::Channel::TagsArmToVc).expect("Unable to read mailbox");

	logok!("Initiated BCM283x driver with framebuffer at {:p}", get_framebuffer().as_ptr());
}

use volatile::Volatile;
pub fn get_framebuffer() -> &'static mut [Volatile<u32>] {
	use core::slice::from_raw_parts_mut;
	unsafe { from_raw_parts_mut(VC_FB_BUFFER.tags.fb_tag.data[0] as *mut Volatile<u32>, (WIDTH * HEIGHT) as usize) }
}

pub fn set_pixel(x: usize, y: usize, color: u32) {
	get_framebuffer()[y * 640 + x].write(color)
}
