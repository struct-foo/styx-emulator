# Intel HEX and ELF Loader Integration Test

This directory contains test data for verifying that the Intel HEX loader and ELF loader produce consistent memory mappings when loading the same binary in different formats.

## Purpose

The Intel HEX loader integration test ensures that:
1. Both loaders produce identical memory regions for the same program
2. The actual binary data loaded matches byte-for-byte
3. Entry points and start addresses are consistent between formats
4. The loaders handle initialized data sections correctly

## Test Program

The test uses a minimal "Hello World" bare metal ARM program (`hello.c`) that:
- Targets ARM Cortex-M4 architecture
- Contains a simple string in the `.rodata` section
- Has initialized data in the `.data` section
- Includes a basic entry point (`_start`) function
- Uses a custom linker script for predictable memory layout

### Memory Layout

The linker script (`link.ld`) defines:
- **Flash memory**: Starting at `0x08000000` (64KB)
- **RAM**: Starting at `0x20000000` (16KB)

This layout is typical for STM32 microcontrollers and provides a realistic (and simple) test case.

## Building the Test Binaries

### Prerequisites

You need the ARM embedded toolchain installed, whether using your system package manager or using
our arm toolchain container at `styx-emulator/util/docker/armtools.Dockerfile`

### Build Commands

```bash
# Build all formats (ELF, Intel HEX, and binary)
make all

# View build information and section details
make info

# Clean build artifacts
make clean

# alternatively, use the armtools image
make docker-build
```

The Makefile produces:
- `hello.elf` - The ELF format binary
- `hello.hex` - Intel HEX format (converted from ELF using objcopy)
- `hello.bin` - Raw binary format
- `hello.lst` - Assembly listing for debugging
- `hello.map` - Linker map file

## References

- [Intel HEX Format Specification](https://en.wikipedia.org/wiki/Intel_HEX)
- [GNU Binutils objcopy Documentation](https://sourceware.org/binutils/docs/binutils/objcopy.html)
