# file : makefile
#
#  Copyright (C) 2018  Joshua Barretto <joshua.s.barretto@gmail.com>
#
#  This program is free software: you can redistribute it and/or modify
#  it under the terms of the GNU General Public License as published by
#  the Free Software Foundation, either version 3 of the License, or
#  (at your option) any later version.
#
#  This program is distributed in the hope that it will be useful,
#  but WITHOUT ANY WARRANTY; without even the implied warranty of
#  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#  GNU General Public License for more details.
#
#  You should have received a copy of the GNU General Public License
#  along with this program.  If not, see <http://www.gnu.org/licenses/>.

# Non-configurable

SRC_ROOT = $(abspath .)
BUILD_ROOT ?= $(SRC_ROOT)/build

# Configurable

KERNEL_EXE ?= $(BUILD_ROOT)/tupai.elf
KERNEL_MAIN = $(SRC_ROOT)/kmain.zig

TARGET_FAMILY ?= x86
TARGET_ARCH ?= i386

TOOL_ASM ?= as
ASM_OBJ = $(BUILD_ROOT)/tupai-asm.o

TOOL_CARGO ?= xargo
CARGO_TARGET = "$(TARGET_ARCH)-tupai"
RUST_LIB = $(BUILD_ROOT)/tupai.a

TOOL_LD ?= ld

# Non-configurable

BUILD_DIRS = $(BUILD_ROOT)

DIR_FAMILY = $(SRC_ROOT)/src/arch/$(TARGET_FAMILY)
DIR_ARCH = $(DIR_FAMILY)/$(TARGET_ARCH)
ifeq ($(TARGET_FAMILY), x86)
	ASM_FILES += $(shell ls $(DIR_FAMILY)/*.{s,S} 2> /dev/null)
	ifeq ($(TARGET_ARCH), i386)
		ASM_FILES += $(shell ls $(DIR_ARCH)/*.{s,S} 2> /dev/null)
	endif
	ifeq ($(TARGET_ARCH), x86_64)
		ASM_FILES += $(shell ls $(DIR_ARCH)/*.{s,S} 2> /dev/null)
	endif
endif

ifeq ($(TARGET_FAMILY), x86)
	ifeq ($(TARGET_ARCH), i386)
		GCC_PREFIX = i686-elf-
	endif
	ifeq ($(TARGET_ARCH), x86_64)
		GCC_PREFIX = x86_64-elf-
	endif
endif

TOOL_ASM_EXEC ?= $(GCC_PREFIX)$(TOOL_ASM)

TOOL_LD_EXEC ?= $(GCC_PREFIX)$(TOOL_LD)
LINK_SCRIPT = $(SRC_ROOT)/arch/$(TARGET_ARCH)/link.ld

ASM_FLAGS = $(addprefix --assembly , $(abspath $(ASM_FILES)))

# Rules

.PHONY: all
all: exe

.PHONY: clean
clean:
	@rm -r -f $(KERNEL_EXE)

$(BUILD_DIRS):
	@mkdir -p $@

.PHONY: exe
exe: $(BUILD_DIRS) asm rust
	@$(TOOL_LD_EXEC) \
		-n --gc-sections \
		-T $(LINK_SCRIPT) \
		-o $(KERNEL_EXE) \
		$(ASM_OBJ) $(RUST_LIB)

.PHONY: asm
asm: $(BUILD_DIRS)
	@$(TOOL_ASM_EXEC) -o $(ASM_OBJ) $(ASM_FILES)

.PHONY: rust
rust: $(BUILD_DIRS)
	@# Why does the following change to RUST_TARGET_PATH work?!
	@RUST_TARGET_PATH=$(shell pwd) $(TOOL_CARGO) build --release --target=$(CARGO_TARGET)
	@cp target/$(CARGO_TARGET)/release/libtupai.a $(RUST_LIB)