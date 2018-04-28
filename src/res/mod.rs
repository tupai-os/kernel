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
	process,
	thread,
	spin::Mutex,
	alloc::{
		boxed::Box,
		Vec,
		BTreeMap,
	},
};

pub type Id = u64;

pub enum Res {
	Process(process::Process),
	Thread(thread::Thread),
}

lazy_static! {
	static ref IDS: Mutex<Id> = Mutex::new(0);
	static ref RES: Mutex<BTreeMap<Id, Res>> = Mutex::new(BTreeMap::new());
}

pub fn create(res: Res) -> Id {
	let nid = {
		// TODO: Prevent ID overflow
		let mut ids = IDS.lock();
		*ids += 1;
		*ids
	};
	RES.lock().insert(nid, res);
	logln!("Created resource with Id {}", nid);
	return nid;
}

pub fn init() {
	RES.lock();
	logok!("Resources initiated");
}
