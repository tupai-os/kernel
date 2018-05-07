// file : shell.rs
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

use llapi::{
	cpu,
	meta,
};
use vdev::tty;
use mem::pfa;
use alloc::{
	String,
	Vec,
};

fn get_chr() -> char {
	loop {
		match tty::input().read() {
			Some(chr) => { return chr; },
			_ => {},
		}
		cpu::halt();
	}
}

fn show_help(args: &[&str]) {
	logln!("Available commands");
	logln!("------------------");
	logln!("  help    Display this message");
	logln!("  info    Show system info");
	logln!("  mmap    Show physical memory map");
}

fn show_info(args: &[&str]) {
	logln!("System Info");
	logln!("-----------");
	logln!("  LLAPI: {}", meta::VARIANT);
	logln!("  Family: {}", meta::FAMILY);
	logln!("  ISA: {}", meta::ISA);
	logln!("  Chipset: {}", meta::CHIPSET);
}

fn show_mmap(args: &[&str]) {
	logln!("Physical Memory Map");
	logln!("------------------");
	pfa::display();
}

pub fn main(args: &[&str]) {
	logln!("\nWelcome to the kernel shell.");
	logln!("Type 'help' for more info.");
	loop {
		log!("> ");

		let mut input: String = String::with_capacity(256);
		loop {
			let c = get_chr();
			match c {
				'\n' => {
					log!("{}", c);
					break;
				},
				'\x08' => {
					if input.len() > 0 {
						log!("{}", c);
						input.pop();
					}
				},
				_ => {
					input.push(c);
					log!("{}", c);
				},
			}
		}

		let args: Vec<&str> = input.split_terminator(' ').collect();
		match args[0] {
			"help" => { show_help(args.as_slice()) },
			"info" => { show_info(args.as_slice()) },
			"mmap" => { show_mmap(args.as_slice()) },
			s => { logln!("Unknown command '{}'", s); },
		}
	}
}
