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

// Publicly visible API
pub mod intrinsic {
	pub use super::{port, exception};
}

// TODO: Make these private
mod boot;
pub mod port;
pub mod exception;

use util::bootcfg::BootCfg;

pub const fn name() -> &'static str { "x86" }

pub fn init(_bootcfg: &BootCfg) {
	exception::init();
	loginfo!("Initiated x86 architecture");
}
