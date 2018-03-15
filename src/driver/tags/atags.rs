
// file : atags.rs
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

use spin::Once;

static INIT: Once<()> = Once::new();

struct TagIterator {
	ptr: usize,
}

#[repr(C, packed)]
struct BasicTag {
	size: u32,
	kind: u32,
}

#[repr(C, packed)]
struct MemTag {
	size: u32,
	kind: u32,
	start: u32,
	bytes: u32,
}

enum Tag {
	BasicTag(&'static BasicTag),
	MemTag(&'static MemTag),
}

use util::math;

impl TagIterator {
	fn from(ptr: *const ()) -> TagIterator {
		use core::mem;
		use arch::base;
		TagIterator {
			ptr: math::align_up(ptr as usize, 3),
		}
	}
}

impl Iterator for TagIterator {
	type Item = Tag;

	fn next(&mut self) -> Option<Tag> {
		let basic_tag = unsafe { &*(self.ptr as *const BasicTag) };
		self.ptr = math::align_down(self.ptr + basic_tag.size as usize, 2); // Increment pointer
		match basic_tag.kind {
			0  => None,
			_  => Some(Tag::BasicTag(unsafe { &*(self.ptr as *const BasicTag) })),
		}
	}
}

use core::fmt;
impl fmt::Display for Tag {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use cstr_core::CStr;
		match self {
			&Tag::BasicTag(t) => write!(f, "Basic tag (kind = {})", t.kind),
			_ => write!(f, "Unknown tag"),
		}
	}
}

pub fn init(tags: *const ()) {
	INIT.call_once(|| {
		loginfo!("Parsing atags at 0x{:X}...", tags as usize);

		for tag in TagIterator::from(tags) {
			logln!("|--> {}", tag);

			match tag {
				Tag::MemTag(t) => {
					use mem::pfa;
					pfa::set_range_kb(t.start as usize >> 10, t.bytes as usize >> 10, pfa::RAM_FREE);
					logok!("Reserved memory")
				}
				_ => {}
			}
		}

		logok!("Parsed atags");
	});
}
