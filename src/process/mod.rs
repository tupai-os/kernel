// file : mod.rs
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

pub mod process;

pub use self::process::{Id, ID_MAX, Process};
use spin::Mutex;
use alloc::btree_map::BTreeMap;

lazy_static! {
	static ref PROCS: Mutex<BTreeMap<Id, Process>> = Mutex::new(BTreeMap::new());
}

bitflags! {
	pub struct Flags: u8 {
		const KERNEL = 0b0001;
	}
}

static ID_COUNTER: Mutex<Id> = Mutex::new(0);

fn get_new_id() -> Id {
	let mut id_counter = ID_COUNTER.lock();
	let id = *id_counter + 1;
	*id_counter = id;
	if id > ID_MAX {
		panic!("Ran out of process identifiers");
	} else {
		id
	}
}

pub fn create(name: &str, flags: Flags) -> Option<Id> {
	let new_id = get_new_id();

	PROCS.lock().insert(new_id, Process::new(new_id, name));

	logln!("Created process '{}' with id {}", name, new_id);
	Some(new_id)
}

pub fn get<'a>(id: Id) -> Option<Process> {
	let id = id;
	match PROCS.lock().get_mut(&id) {
		Some(p) => Some(p.clone()),
		None => None
	}
}

pub fn init() {
	logok!("Initiated processes");
}
