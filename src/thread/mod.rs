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

use {
	res,
	alloc::string,
};

pub struct Thread {
	pub id: res::Id,
	pub name: string::String,
}

impl Thread {
	pub fn new(name: &str) -> Thread {
		Thread {
			id: 0,
			name: string::String::from(name),
		}
	}
}

#[derive(Debug)]
pub enum ThreadErr {}

pub fn create(name: &str) -> Result<res::Id, ThreadErr> {
	return Ok(
		res::create(res::Res::Thread(Thread {
			id: 0,
			name: string::String::from(name),
		}))
	);
}

// pub mod thread;

// pub use self::thread::{Id, ID_MAX, Thread, Stack};
// use spin::Mutex;
// use alloc::{boxed::Box, Vec, BTreeMap};

// lazy_static! {
//	 static ref THREADS: Mutex<BTreeMap<Id, Thread>> = Mutex::new(BTreeMap::new());
// }

// static ID_COUNTER: Mutex<Id> = Mutex::new(0);

// fn get_new_id() -> Id {
//	 let mut id_counter = ID_COUNTER.lock();
//	 let id = *id_counter + 1;
//	 *id_counter = id;
//	 if id > ID_MAX {
//		 panic!("Ran out of thread identifiers");
//	 } else {
//		 id
//	 }
// }

// pub fn create(name: &str, entry: fn() -> i32) -> Option<Id> {
//	 let new_id = get_new_id();

//	 let stack: Stack = Box::new(Vec::with_capacity(1024));

//	 THREADS.lock().insert(new_id, Thread::new(new_id, name, entry, stack));

//	 logln!("Created thread '{}' with id {}", name, new_id);
//	 Some(new_id)
// }

// pub fn init() {
//	 THREADS.lock();
//	 logok!("Threads initiated");
// }
