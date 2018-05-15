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

use std::env;

fn write_cfg(key: &str, val: &str) {
	println!("cargo:rustc-cfg={}=\"{}\"", key, val);
}

fn write_feature(key: &str) {
	println!("cargo:rustc-cfg={}=\"true\"", key);
}

fn main() {
	// Testing only
	match env::var("TUPAI_TARGET").unwrap().as_ref() {
		"x64" => {
			write_cfg("arch_llapi", "x64");

			write_cfg("arch_family", "x86");
			write_cfg("arch_cpu", "amd64");
			write_cfg("arch_chipset", "ibmpc");
			write_cfg("arch_tags", "multiboot");

			write_cfg("log_driver", "video_vga");
			write_feature("driver_serial_com");
			write_feature("driver_video_vga");
		},
		"i386" => {
			write_cfg("arch_llapi", "i386");

			write_cfg("arch_family", "x86");
			write_cfg("arch_cpu", "ia32");
			write_cfg("arch_chipset", "ibmpc");
			write_cfg("arch_tags", "multiboot");

			write_cfg("log_driver", "video_vga");
			write_feature("driver_serial_com");
			write_feature("driver_video_vga");
		},
		"rpi2" => {
			write_cfg("arch_llapi", "rpi2");

			write_cfg("arch_family", "arm");
			write_cfg("arch_cpu", "a32");
			write_cfg("arch_chipset", "bcm2835");
			write_cfg("arch_tags", "atags");

			write_cfg("log_driver", "serial_uart");
			write_feature("driver_serial_uart");
			write_feature("driver_video_bcm2835");
		},
		other => {
			panic!("Invalid target '{}' specified", other);
		}
	}
}
