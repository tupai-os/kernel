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

KERNEL_ELF ?= $(BUILD_ROOT)/tupai.elf

TOOL_CARGO ?= xargo
RUST_LIB = $(BUILD_ROOT)/tupai.a

CARGO_TARGET = $(CFG_arch_isa)-tupai
CARGO_BYPRODUCT = target

TOOL_LINKER ?= ld

# Non-configurable

BUILD_DIRS = $(BUILD_ROOT)

# Find all assembly
DIR_FAMILY = $(SRC_ROOT)/src/arch/$(CFG_arch_base)
DIR_ARCH = $(DIR_FAMILY)/$(CFG_arch_isa)

LINK_SCRIPT = $(SRC_ROOT)/arch/$(CFG_arch_isa)/link.ld

SYMBOLS = $(BUILD_ROOT)/tupai.symb
SYMBOL_CMD = objdump --wide --syms $(KERNEL_ELF) | grep -P '^[0-9A-Fa-f]+\s.*\s[a-zA-Z_][a-zA-Z0-9_]+$$' | sed -r 's/^(\S+)\s+.*\s+(\S+)$$/\1 \2/' | sort > $(SYMBOLS)

# Rules

.PHONY: all
all: exe

.PHONY: clean
clean:
	@rm -r -f $(KERNEL_ELF) $(CARGO_BYPRODUCT)

$(BUILD_DIRS):
	@echo "Creating build directories..."
	@mkdir -p $@

.PHONY: check
check:
	@echo "Invoking cargo check..."
	@RUST_TARGET_PATH=$(shell pwd) $(TOOL_CARGO) \
		check \
		--debug \
		--target=$(CARGO_TARGET) \
		--features "$(CARGO_FEATURES)"

.PHONY: exe
exe: $(BUILD_DIRS) rust
	@echo "Linking kernel..."
	@$(TOOL_LINKER) \
		-n --gc-sections \
		-T $(LINK_SCRIPT) \
		-o $(KERNEL_ELF) \
		$(ASM_OBJ) $(RUST_LIB)

.PHONY: symbols
symbols: exe
	@echo "Generating symbols..."
	@$(SYMBOL_CMD)

.PHONY: rust
rust: $(BUILD_DIRS)
	@echo "Invoking cargo build..."
	@RUST_TARGET_PATH=$(shell pwd) RUSTFLAGS="" $(TOOL_CARGO) \
		build \
		--release \
		--target=$(CARGO_TARGET) \
		--features "$(CARGO_FEATURES)"
	@cp target/$(CARGO_TARGET)/release/libtupai.a $(RUST_LIB)
