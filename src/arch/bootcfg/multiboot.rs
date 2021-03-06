// file : multiboot.rs
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

use util::bootcfg::{BootCfg, Module, Args};
use util::math::align_up;
use cstr_core::CStr;
use core::mem;

struct TagIterator {
	ptr: usize,
}

#[repr(C, packed)]
struct FixedTag {
	total_size: u32,
	_unused: u32,
}

#[repr(C, packed)]
struct BasicTag {
	kind: u32,
	size: u32,
}

#[repr(C, packed)]
struct MemoryTag {
	kind: u32,
	size: u32,
	lower: u32,
	upper: u32,
}

#[repr(C, packed)]
struct BiosDeviceTag {
	kind: u32,
	size: u32,
	biosdev: u32,
	partition: u32,
	sub_partition: u32,
}

#[repr(C, packed)]
struct BootCommandTag {
	kind: u32,
	size: u32,
	command: i8,
}

#[repr(C, packed)]
struct ModuleTag {
	kind: u32,
	size: u32,
	mod_start: u32,
	mod_end: u32,
	command: i8,
}

#[repr(C, packed)]
struct ElfSymbolsTag {
	kind: u32,
	size: u32,
	num: u16,
	entsize: u16,
	shndx: u16,
	_unused: u16,
	// <section_headers>
}

#[repr(C, packed)]
struct MemoryMapEntry {
	base_addr: u64,
	length: u64,
	kind: u32,
	_unused: u32,
}

#[repr(C, packed)]
struct MemoryMapTag {
	kind: u32,
	size: u32,
	entry_size: u32,
	entry_version: u32,
	entries: MemoryMapEntry,
}

#[repr(C, packed)]
struct BootloaderNameTag {
	kind: u32,
	size: u32,
	name: i8,
}

#[repr(C, packed)]
struct ApmTableTag {
	kind: u32,
	size: u32,
	version: u16,
	cseg: u16,
	offset: u32,
	cseg_16: u16,
	dseg: u16,
	flags: u16,
	cseg_len: u16,
	cseg_16_len: u16,
	dseg_len: u16,
}

#[repr(C, packed)]
struct VbeTag {
	kind: u32,
	size: u32,
	mode: u16,
	interface_seg: u16,
	interface_off: u16,
	interface_len: u16,
	control_info: [u8; 512],
	mode_info: [u8; 256],
}

#[repr(C, packed)]
struct FramebufferTag {
	kind: u32,
	size: u32,
	addr: u64,
	pitch: u32,
	width: u32,
	height: u32,
	bpp: u8,
	_type: u8,
	_unused: u8,
	// <color_info>
}

#[repr(C, packed)]
struct Efi32TableTag {
	kind: u32,
	size: u32,
	pointer: u32,
}

#[repr(C, packed)]
struct Efi64TableTag {
	kind: u32,
	size: u32,
	pointer: u64,
}

enum Tag {
	BasicTag(&'static BasicTag),
	MemoryTag(&'static MemoryTag),
	BiosDeviceTag(&'static BiosDeviceTag),
	BootCommandTag(&'static BootCommandTag),
	ModuleTag(&'static ModuleTag),
	ElfSymbolsTag(&'static ElfSymbolsTag),
	MemoryMapTag(&'static MemoryMapTag),
	BootloaderNameTag(&'static BootloaderNameTag),
	ApmTableTag(&'static ApmTableTag),
	VbeTag(&'static VbeTag),
	FramebufferTag(&'static FramebufferTag),
	Efi32TableTag(&'static Efi32TableTag),
	Efi64TableTag(&'static Efi64TableTag),
}

impl TagIterator {
	fn from(ptr: *const ()) -> TagIterator {
		let _fixed_tag = unsafe { &*(ptr as *const FixedTag) };
		TagIterator {
			ptr: align_up(ptr as usize + mem::size_of::<FixedTag>(), 3),
		}
	}
}

impl Iterator for TagIterator {
	type Item = Tag;

	fn next(&mut self) -> Option<Tag> {
		let basic_tag = unsafe { &*(self.ptr as *const BasicTag) };

		let tag = match basic_tag.kind {
			0  => None,
			1  => Some(Tag::BootCommandTag(unsafe { &*(self.ptr as *const BootCommandTag) })),
			2  => Some(Tag::BootloaderNameTag(unsafe { &*(self.ptr as *const BootloaderNameTag) })),
			3  => Some(Tag::ModuleTag(unsafe { &*(self.ptr as *const ModuleTag) })),
			4  => Some(Tag::MemoryTag(unsafe { &*(self.ptr as *const MemoryTag) })),
			5  => Some(Tag::BiosDeviceTag(unsafe { &*(self.ptr as *const BiosDeviceTag) })),
			6  => Some(Tag::MemoryMapTag(unsafe { &*(self.ptr as *const MemoryMapTag) })),
			7  => Some(Tag::VbeTag(unsafe { &*(self.ptr as *const VbeTag) })),
			8  => Some(Tag::FramebufferTag(unsafe { &*(self.ptr as *const FramebufferTag) })),
			9  => Some(Tag::ElfSymbolsTag(unsafe { &*(self.ptr as *const ElfSymbolsTag) })),
			10 => Some(Tag::ApmTableTag(unsafe { &*(self.ptr as *const ApmTableTag) })),
			11 => Some(Tag::Efi32TableTag(unsafe { &*(self.ptr as *const Efi32TableTag) })),
			12 => Some(Tag::Efi64TableTag(unsafe { &*(self.ptr as *const Efi64TableTag) })),
			_  => Some(Tag::BasicTag(unsafe { &*(self.ptr as *const BasicTag) })),
		};

		self.ptr = align_up(self.ptr + basic_tag.size as usize, 3); // Increment pointer
		return tag;
	}
}

use core::fmt;
impl fmt::Display for Tag {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Tag::BasicTag(_t) => write!(f, "Basic tag"),
			&Tag::BootCommandTag(t) => unsafe { write!(f,
				"Boot command tag (command = \"{}\")",
				CStr::from_ptr(&t.command).to_str().unwrap_or(""),
			) },
			&Tag::BootloaderNameTag(t) => unsafe { write!(f,
				"Bootloader name tag (name = \"{}\")",
				CStr::from_ptr(&t.name).to_str().unwrap_or("<none>"),
			) },
			&Tag::ModuleTag(t) => unsafe { write!(f,
				"Module tag (mod_start = {}, mod_end = {}, command = \"{}\")",
				t.mod_start,
				t.mod_end,
				CStr::from_ptr(&t.command).to_str().unwrap(),
			) },
			&Tag::MemoryMapTag(t) => unsafe { write!(f,
				"Memory map tag (entry_size = {}, entry_version = {})",
				t.entry_size,
				t.entry_version,
			) },
			&Tag::BiosDeviceTag(t) => unsafe { write!(f,
				"BIOS device tag (biosdev = {}, partition = {}, sub_partition = {})",
				t.biosdev,
				t.partition,
				t.sub_partition,
			) },
			&Tag::MemoryTag(t) => unsafe { write!(f,
				"Memory tag (lower = {}, upper = {})",
				t.lower,
				t.upper,
			) },
			&Tag::VbeTag(t) => unsafe { write!(f,
				"VBE tag (mode = {}, interface_seg = {}, interface_off = {}, interface_len = {})",
				t.mode,
				t.interface_seg,
				t.interface_off,
				t.interface_len,
			) },
			&Tag::FramebufferTag(t) => unsafe { write!(f,
				"Framebuffer tag (addr = 0x{:X}, pitch = {}, width = {}, height = {}, bpp = {}, type = {})",
				t.addr,
				t.pitch,
				t.width,
				t.height,
				t.bpp,
				t._type,
			) },
			&Tag::ElfSymbolsTag(t) => unsafe { write!(f,
				"ELF symbols tag (num = {}, entsize = {}, shndx = {})",
				t.num,
				t.entsize,
				t.shndx,
			) },
			&Tag::ApmTableTag(t) => unsafe { write!(f,
				"APM table tag (version = {}, cseg = {}, offset = {}, cseg_16 = {}, dseg = {}, flags = {}, cseg_len = {}, cseg_16_len = {}, dseg_len = {})",
				t.version,
				t.cseg,
				t.offset,
				t.cseg_16,
				t.dseg,
				t.flags,
				t.cseg_len,
				t.cseg_16_len,
				t.dseg_len,
			) },
			_ => write!(f, "Unknown tag"),
		}
	}
}

pub const fn name() -> &'static str { "Multiboot" }

pub fn parse(tags: *const ()) -> BootCfg {
	loginfo!("Parsing Multiboot tags at 0x{:X}...", tags as usize);

	let mut data = BootCfg::empty();
	for tag in TagIterator::from(tags) {
		logln!("|--> {}", tag);
		match tag {
			Tag::MemoryTag(t) => data.mem_ram = 1024 + t.upper as usize,
			Tag::BootCommandTag(t) => {
				let cmd_str = unsafe { CStr::from_ptr(&t.command).to_str().unwrap_or("") };
				data.args.extend(cmd_str.split_terminator(' ').collect::<Args>());
			},
			Tag::ModuleTag(t) => {
				let cmd_str = unsafe { CStr::from_ptr(&t.command).to_str().unwrap_or("") };
				data.modules.try_push(Module::new(
					t.mod_start as usize,
					(t.mod_end - t.mod_start) as usize,
					cmd_str.split_terminator(' ').collect::<Args>(),
				)).unwrap_or_else(|e|{
					panic!("Too many boot modules! ({:?})", e);
				});
			},
			_ => {}
		}
	}

	logok!("Parsed Multiboot tags");
	return data;
}
