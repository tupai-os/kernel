// file : chipset.rs
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

pub mod intrinsic {
	pub use super::{pic, pit, kbd, regions};
}

// TODO: Make these private
pub mod pic;
pub mod pit;
pub mod kbd;
mod spurious;
pub mod regions;

use util::bootcfg::BootCfg;

pub const fn name() -> &'static str { "IBM PC" }

pub fn init(_bootcfg: &BootCfg) {
	pic::init();
	spurious::init();
	pit::init();
	kbd::init();

	loginfo!("Initiated IBM-PC chipset architecture");
}
