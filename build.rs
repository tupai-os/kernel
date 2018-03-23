// file : build.rs
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

fn write_cfg(key: &str, val: &str) {
	println!("cargo:rustc-cfg={}=\"{}\"", key, val);
}

fn write_feature(key: &str) {
	println!("cargo:rustc-cfg={}=\"true\"", key);
}

fn main() {
	// Testing only
	write_cfg("arch_hal",     "x64");   // i386, armv7, armv8
	write_cfg("arch_family",  "x86");   // arm
	write_cfg("arch_isa",     "amd64"); // ia32, a32, a64
	write_cfg("arch_chipset", "ibmpc"); // bcm2836

	write_feature("log_serial_com");
	write_feature("driver_serial_com");
}
