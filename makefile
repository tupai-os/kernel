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
KERNEL_MAIN = $(SRC_ROOT)/kmain.zig

ifndef CFG_arch_base
  $(error CFG_arch_base must be defined)
endif
ifndef CFG_arch_isa
  $(error CFG_arch_isa must be defined)
endif
ifndef CFG_drivers_ttyout
  $(error CFG_drivers_ttyout must be defined)
endif
ifndef CFG_drivers_ttyin
  $(error CFG_drivers_ttyin must be defined)
endif
ifndef CFG_drivers_tags
  $(error CFG_drivers_tags must be defined)
endif

TOOL_ASM ?= as
ASM_OBJ = $(BUILD_ROOT)/tupai-asm.o

TOOL_CARGO ?= xargo
RUST_LIB = $(BUILD_ROOT)/tupai.a

CARGO_TARGET = $(CFG_arch_isa)-tupai
CARGO_BYPRODUCT = target
CARGO_FEATURES = \
  arch_base_$(CFG_arch_base) \
  arch_isa_$(CFG_arch_isa) \
  driver_ttyout_$(CFG_drivers_ttyout) \
  driver_ttyin_$(CFG_drivers_ttyin) \
  driver_tags_$(CFG_drivers_tags) \
  $(foreach vd, $(CFG_drivers_video), driver_video_$(vd)) \
  $(foreach sd, $(CFG_drivers_serial), driver_serial_$(sd))
ifdef CFG_board_model
  CARGO_FEATURES += board_model_$(CFG_board_model)
endif

TOOL_LD ?= ld

# Non-configurable

BUILD_DIRS = $(BUILD_ROOT)

# Find all assembly
DIR_FAMILY = $(SRC_ROOT)/src/arch/$(CFG_arch_base)
DIR_ARCH = $(DIR_FAMILY)/$(CFG_arch_isa)

ASM_FILES += $(shell ls $(DIR_FAMILY)/*.{s,S} 2> /dev/null)
ASM_FILES += $(shell ls $(DIR_FAMILY)/boot/*.{s,S} 2> /dev/null)

ASM_FILES += $(shell ls $(DIR_ARCH)/*.{s,S} 2> /dev/null)
ASM_FILES += $(shell ls $(DIR_ARCH)/boot/*.{s,S} 2> /dev/null)

ASM_FLAGS ?=
ifeq ($(CFG_arch_base), x86)
	ifeq ($(CFG_arch_isa), i386)
		GCC_PREFIX = i686-elf-
	endif
	ifeq ($(CFG_arch_isa), x86_64)
		GCC_PREFIX = x86_64-elf-
	endif
endif
ifeq ($(CFG_arch_base), arm)
	ifeq ($(CFG_arch_isa), armv7)
		GCC_PREFIX = arm-none-eabi-
		ASM_FLAGS += -mcpu=arm1176jzf-s
	endif
	ifeq ($(CFG_arch_isa), armv8)
		GCC_PREFIX = aarch64-none-eabi-
	endif
endif

TOOL_ASM_EXEC ?= $(GCC_PREFIX)$(TOOL_ASM)

TOOL_LD_EXEC ?= $(GCC_PREFIX)$(TOOL_LD)
LINK_SCRIPT = $(SRC_ROOT)/arch/$(CFG_arch_isa)/link.ld

SYMBOLS = $(BUILD_ROOT)/tupai.symb
SYMBOL_CMD = objdump --wide --syms $(KERNEL_ELF) | grep -P '^[0-9A-Fa-f]+\s.*\s[a-zA-Z_][a-zA-Z0-9_]+$$' | sed -r 's/^(\S+)\s+.*\s+(\S+)$$/\1 \2/' | sort > $(SYMBOLS)

# Rules

.PHONY: all
all: exe symbols

.PHONY: clean
clean:
	@rm -r -f $(KERNEL_ELF) $(CARGO_BYPRODUCT)

$(BUILD_DIRS):
	@mkdir -p $@

.PHONY: check
check:
	@# Why does the following change to RUST_TARGET_PATH work?!
	@RUST_TARGET_PATH=$(shell pwd) $(TOOL_CARGO) \
		check \
		--debug \
		--target=$(CARGO_TARGET) \
		--features "$(CARGO_FEATURES)"

.PHONY: exe
exe: $(BUILD_DIRS) rust
	@$(TOOL_LD_EXEC) \
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
	@# Why does the following change to RUST_TARGET_PATH work?!
	@RUST_TARGET_PATH=$(shell pwd) RUSTFLAGS="" $(TOOL_CARGO) \
		build \
		--release \
		--target=$(CARGO_TARGET) \
		--features "$(CARGO_FEATURES)"
	@cp target/$(CARGO_TARGET)/release/libtupai.a $(RUST_LIB)
