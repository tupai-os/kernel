// file : tar.rs
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

use util::math::align_up;
use alloc::String;
use core::slice;

#[derive(Copy, Clone)]
pub struct Tar {
	start: *const File,
}

pub struct File {
	filename: [u8; 100],
	filemode: [u8; 8],
	user: [u8; 8],
	group: [u8; 8],
	size: [u8; 12],
	last_change: [u8; 12],
	checksum: [u8; 8],
	filetype: u8,
}

pub struct Iter {
	file: &'static File,
}

impl Tar {
	pub unsafe fn from(ptr: usize) -> Tar {
		Tar {
			start: ptr as *const File,
		}
	}
}

impl IntoIterator for Tar {
	type Item = &'static File;
	type IntoIter = Iter;

	fn into_iter(self) -> Self::IntoIter {
		Iter {
			file: unsafe { &*self.start },
		}
	}
}

impl Iterator for Iter {
	type Item = &'static File;

	fn next(&mut self) -> Option<Self::Item> {
		let offset = align_up(512 + self.file.size(), 9);
		let file = if self.file.valid() {
			Some(self.file)
		} else {
			None
		};
		self.file = unsafe { &*((self.file as *const File as usize + offset) as *const File) };
		return file;
	}
}

impl File {
	// TODO: Verify this works with the tar standard in ALL CASES
	pub fn size(&'static self) -> usize {
		let mut s = String::new();
		for c in self.size.iter() {
			match c {
				b'\0' => break,
				_ => s.push(*c as char),
			};
		}
		return usize::from_str_radix(&s, 8).unwrap_or(0); // <-- TODO: Should we be doing this?
	}

	fn valid(&'static self) -> bool {
		self.filename[0] != 0
	}

	pub fn name(&'static self) -> String {
		let mut s = String::new();
		for c in self.filename.iter() {
			match c {
				b'\0' => break,
				_ => s.push(*c as char),
			};
		}
		return s;
	}

	pub fn data(&'static self) -> &[u8] {
		unsafe { slice::from_raw_parts((self as *const File as usize + 512) as *const u8, self.size()) }
	}
}
