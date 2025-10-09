# SPDX-License-Identifier: BSD-2-Clause
# Makefile for building hello world test binary in both ELF and Intel hex formats

# Cross-compilation toolchain for ARM
# Users may need to install arm-none-eabi-gcc toolchain
CROSS_COMPILE ?= arm-none-eabi-
CC = $(CROSS_COMPILE)gcc
OBJCOPY = $(CROSS_COMPILE)objcopy
OBJDUMP = $(CROSS_COMPILE)objdump
SIZE = $(CROSS_COMPILE)size

# Compiler flags
CFLAGS = -mcpu=cortex-m4 \
         -mthumb \
         -mfloat-abi=soft \
         -nostdlib \
         -nostartfiles \
         -ffreestanding \
         -fno-builtin \
         -Wall \
         -O2 \
         -g

# Linker flags
LDFLAGS = -T link.ld \
          -Wl,-Map=hello.map \
          -Wl,--gc-sections

# Target files
TARGET = hello
ELF_FILE = $(TARGET).elf
HEX_FILE = $(TARGET).hex
BIN_FILE = $(TARGET).bin
MAP_FILE = $(TARGET).map
LST_FILE = $(TARGET).lst

# Source files
SRCS = hello.c

# Build rules
.PHONY: all clean info build-docker docker-build

all: $(ELF_FILE) $(HEX_FILE) $(BIN_FILE) $(LST_FILE) info

$(ELF_FILE): $(SRCS) link.ld
	$(CC) $(CFLAGS) $(LDFLAGS) $(SRCS) -o $@
	@echo "Built ELF file: $@"

$(HEX_FILE): $(ELF_FILE)
	$(OBJCOPY) -O ihex --keep-section=.deadbeef --keep-section=.counter --keep-section=.data $< $@
	@echo "Generated Intel HEX file: $@"

$(BIN_FILE): $(ELF_FILE)
	$(OBJCOPY) -O binary $< $@
	@echo "Generated binary file: $@"

$(LST_FILE): $(ELF_FILE)
	$(OBJDUMP) -h -S $< > $@
	@echo "Generated listing file: $@"

build-docker:
	@echo "Building Docker image for ARM cross-compilation..."
	make -C ${STYX_ROOT}/util/docker armtools-image

docker-build: build-docker
	docker run --rm -v $(PWD):/src -w /src armtools:latest make all

info: $(ELF_FILE)
	@echo ""
	@echo "=== Binary Information ==="
	$(SIZE) $(ELF_FILE)
	@echo ""
	@echo "=== Section Headers ==="
	$(OBJDUMP) -h $(ELF_FILE)
	@echo ""
	@echo "=== Files Generated ==="
	@ls -lh $(ELF_FILE) $(HEX_FILE) $(BIN_FILE) 2>/dev/null || true
	@echo ""

clean:
	rm -f $(ELF_FILE) $(HEX_FILE) $(BIN_FILE) $(MAP_FILE) $(LST_FILE)
	@echo "Cleaned build artifacts"

# Help target
help:
	@echo "Hello World Test Binary Build System"
	@echo "====================================="
	@echo "Targets:"
	@echo "  all   - Build ELF, Intel HEX, and binary files"
	@echo "  clean - Remove all generated files"
	@echo "  info  - Display size and section information"
	@echo "  help  - Show this help message"
	@echo "  build-docker - Build Docker image for ARM cross-compilation"
	@echo "  docker-build - Build using Docker with ARM cross-compilation"
	@echo ""
	@echo "Prerequisites:"
	@echo "  arm-none-eabi-gcc toolchain must be installed"
	@echo "  On macOS: brew install arm-none-eabi-gcc"
	@echo "  On Ubuntu: apt-get install gcc-arm-none-eabi"
