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

pub mod env;

use env::env::{EnvId, ENVID_MAX, Env};
use spin::Mutex;

use alloc::btree_map::BTreeMap;
lazy_static! {
	static ref ENVS: Mutex<BTreeMap<EnvId, Env>> = Mutex::new(BTreeMap::new());
}

bitflags! {
	pub struct Flags: u8 {
		const KERNEL = 0b0001;
	}
}

static ENVID_COUNTER: Mutex<EnvId> = Mutex::new(0);

fn get_new_id() -> EnvId {
	let mut envid_counter = ENVID_COUNTER.lock();
	let id = *envid_counter + 1;
	*envid_counter = id;
	id
}

pub fn create(name: &str, flags: Flags) -> Option<EnvId> {
	let new_id = get_new_id();

	ENVS.lock().insert(new_id, Env::new(new_id, name));

	logln!("Created environment '{}' with id {}", name, new_id);
	Some(new_id)
}

pub fn get<'a>(id: EnvId) -> Option<Env> {
	let id = id;
	match ENVS.lock().get_mut(&id) {
		Some(e) => Some(e.clone()),
		None => None
	}
}

pub fn init() {
	ENVS.lock();
	logok!("Initiated environments");
}
