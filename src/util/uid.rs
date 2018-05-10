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
use alloc::{
	arc::{Arc, Weak},
	BTreeMap,
	Vec,
};

pub type Uid = i64;

pub struct Tracker<T> {
	items: IrqLock<BTreeMap<Uid, Arc<T>>>,
}

// TODO: Code duplication? Do something about this
pub struct WeakTracker<T> {
	items: IrqLock<BTreeMap<Uid, Weak<T>>>,
}

lazy_static! {
	static ref UID_COUNT: IrqLock<Uid> = IrqLock::new(0);
}

pub fn new_uid() -> Uid {
	return {
		// TODO: Prevent ID overflow
		let mut uid_count = UID_COUNT.lock();
		*uid_count += 1;
		*uid_count
	};
}

impl<'a, T> Tracker<T> {
	pub fn new() -> Tracker<T> {
		Tracker::<T> {
			items: IrqLock::new(BTreeMap::new()),
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

	pub fn items(&'a self) -> &'a IrqLock<BTreeMap<Uid, Arc<T>>> {
		&self.items
	}
}

impl<'a, T> WeakTracker<T> {
	pub fn new() -> WeakTracker<T> {
		WeakTracker::<T> {
			items: IrqLock::new(BTreeMap::new()),
		}
	}

	pub fn emplace(&self, item: T) -> (Uid, Arc<T>) {
		let uid = new_uid();
		let arc = Arc::new(item);
		self.items.lock().insert(uid, Arc::downgrade(&arc));
		return (uid, arc);
	}

	pub fn emplace_with_uid(&self, uid: Uid, item: T) -> (Uid, Arc<T>) {
		let arc = Arc::new(item);
		self.items.lock().insert(uid, Arc::downgrade(&arc));
		return (uid, arc);
	}

	pub fn get(&self, uid: Uid) -> Option<Arc<T>> {
		match self.items.lock().get(&uid) {
			Some(arc) => arc.upgrade(),
			_ => None,
		}
	}

	pub fn uids(&self) -> Vec<Uid> {
		return self.items.lock().keys().cloned().collect();
	}

	pub fn items(&'a self) -> &'a IrqLock<BTreeMap<Uid, Weak<T>>> {
		&self.items
	}
}
