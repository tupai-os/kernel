// file : path.rs
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

use alloc::{
	vec,
	Vec,
	String,
};

pub struct Path {
	parts: Vec<String>,
}

impl Path {
	pub fn new() -> Path {
		Path {
			parts: Vec::new(),
		}
	}

	pub fn from(path: &str) -> Path {
		Path {
			parts: path.split_terminator("/").map(|s| String::from(s)).collect(),
		}
	}
}

impl IntoIterator for Path {
	type Item = String;
	type IntoIter = vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.parts.into_iter()
	}
}
