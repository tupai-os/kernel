
// file : multiboot.rs
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
	kind: u32,
	size: u32,
}

enum Tag {
	BasicTag(&'static BasicTag),
}

impl TagIterator {
	fn from(ptr: *const ()) -> TagIterator {
		TagIterator {
			ptr: (ptr as usize) & (!0x7),
		}
	}
}

impl Iterator for TagIterator {
	type Item = Tag;

	fn next(&mut self) -> Option<Tag> {
		logln!("Tag at 0x{:X}", self.ptr);
		let basic_tag = unsafe { &*(self.ptr as *const BasicTag) };
		if basic_tag.kind == 0 {
			None
		} else {
			self.ptr += (basic_tag.size as usize) & (!0x7);
			//Some(Tag::BasicTag(unsafe { &*(self.ptr as *const BasicTag) }));
			None
		}
	}
}

pub fn init(tags: *const ()) {
	INIT.call_once(|| {
		for it in TagIterator::from(tags) {
			logln!("Found tag!");
			break
		}

		logok!("Parsed Multiboot parameters");
	});
}
