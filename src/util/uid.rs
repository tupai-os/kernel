// file : uid.rs
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

use util::IrqLock;
use spin::Mutex;
use alloc::{
	arc::Arc,
	BTreeMap,
	Vec,
};

pub type Uid = i64;

// TODO: Make this IRQ-safe
pub struct Tracker<T> {
	items: Mutex<BTreeMap<Uid, Arc<T>>>,
}

lazy_static! {
	static ref UID_COUNT: Mutex<Uid> = Mutex::new(0);
}

pub fn new_uid() -> Uid {
	let _lock = IrqLock::new();
	return {
		// TODO: Prevent ID overflow
		let mut uid_count = UID_COUNT.lock();
		*uid_count += 1;
		*uid_count
	};
}

impl<T> Tracker<T> {
	pub fn new() -> Tracker<T> {
		Tracker::<T> {
			items: Mutex::new(BTreeMap::new()),
		}
	}

	pub fn emplace(&self, item: T) -> (Uid, Arc<T>) {
		let uid = new_uid();
		let arc = Arc::new(item);
		self.items.lock().insert(uid, arc.clone());
		return (uid, arc);
	}

	pub fn emplace_with_uid(&self, uid: Uid, item: T) -> (Uid, Arc<T>) {
		let arc = Arc::new(item);
		self.items.lock().insert(uid, arc.clone());
		return (uid, arc);
	}

	pub fn get(&self, uid: Uid) -> Option<Arc<T>> {
		match self.items.lock().get(&uid) {
			Some(arc) => Some(arc.clone()),
			_ => None,
		}
	}

	pub fn uids(&self) -> Vec<Uid> {
		return self.items.lock().keys().cloned().collect();
	}
}
