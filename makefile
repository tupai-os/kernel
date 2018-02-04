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

TOOL_ZIG ?= zig

# Non-configurable

BUILD_DIRS = $(BUILD_ROOT)

DIR_FAMILY = $(SRC_ROOT)/arch/$(TARGET_FAMILY)
DIR_ARCH = $(DIR_FAMILY)/$(TARGET_ARCH)
ifeq ($(TARGET_FAMILY), x86)
	ASM_FILES += $(shell ls $(DIR_FAMILY)/*.{s,S} 2> /dev/null)
	ifeq ($(TARGET_ARCH), i386)
		LINK_SCRIPT = $(DIR_ARCH)/link.ld
		ASM_FILES += $(shell ls $(DIR_ARCH)/*.{s,S} 2> /dev/null)
	endif
endif

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
exe: $(BUILD_DIRS)
	@$(TOOL_ZIG) build-exe \
		--cache-dir $(BUILD_ROOT)/_zig-cache \
		--output $(KERNEL_EXE) \
		--target-arch $(TARGET_ARCH) \
		--target-environ gnu \
		--target-os freestanding \
		--linker-script $(LINK_SCRIPT) \
		--release-fast \
		$(ASM_FLAGS) \
		$(KERNEL_MAIN)