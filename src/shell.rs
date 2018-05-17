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

use llapi;
use vdev::tty;
use mem::pfa;
use process;
use vfs;
use alloc::{
	string::ToString,
	String,
	Vec,
};

fn get_chr() -> char {
	loop {
		match tty::input().read() {
			Some(chr) => { return chr; },
			_ => {},
		}
		llapi::cpu::irq::await();
	}
}

fn show_help(_args: &[&str]) {
	logln!("Available commands");
	logln!("------------------");
	logln!("  help    Display this message");
	logln!("  info    Show system info");
	logln!("  mmap    Show physical memory map");
	logln!("  proc    Show process and threads");
	logln!("  tree    Show filesystem tree");
}

fn show_info(_args: &[&str]) {
	logln!("System Info");
	logln!("-----------");
	logln!("  LLAPI: {}", llapi::name());
	logln!("  Family: {}", llapi::family::name());
	logln!("  CPU: {}", llapi::cpu::name());
	logln!("  Chipset: {}", llapi::chipset::name());
}

fn show_mmap(_args: &[&str]) {
	logln!("Physical Memory Map");
	logln!("-------------------");
	pfa::display();
}

fn show_proc(_args: &[&str]) {
	logln!("Process List");
	logln!("------------");
	for proc in process::list() {
		logln!("Process (uid = {}, name = {})",
			proc.uid(),
			proc.name().unwrap_or("<none>".to_string())
		);
		for thread in (*proc.threads().unwrap()).iter() {
			logln!("|-> Thread (uid = {}, name = {})",
				thread.uid(),
				thread.name().unwrap_or("<none>".to_string())
			);
		}
	}
}

fn show_tree(_args: &[&str]) {
	logln!("Filesystem Tree");
	logln!("---------------");
	vfs::display();
}

pub fn main() {
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
			"proc" => { show_proc(args.as_slice()) },
			"tree" => { show_tree(args.as_slice()) },
			s => { logln!("Unknown command '{}'", s); },
		}
	}
}
