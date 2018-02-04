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

KERNEL_EXE = $(BUILD_ROOT)/tupai
KERNEL_MAIN = $(SRC_ROOT)/kmain.zig

TARGET_FAMILY ?= x86
TARGET_ARCH ?= i386

TOOL_ZIG ?= zig

# Non-configurable

BUILD_DIRS = $(BUILD_ROOT)

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
		--output $(KERNEL_EXE) \
		--target-arch i386 \
		--target-environ gnu \
		--target-os freestanding \
		$(KERNEL_MAIN)