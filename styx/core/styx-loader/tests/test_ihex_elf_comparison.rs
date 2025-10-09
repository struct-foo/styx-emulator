// SPDX-License-Identifier: BSD-2-Clause
//! Integration test comparing Intel HEX and ELF loader outputs
//!
//! This test ensures that the Intel HEX loader and ELF loader produce
//! consistent memory mappings when loading the same binary in different formats.

use std::borrow::Cow;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use styx_loader::{ElfLoader, IhexLoader, Loader};

/// Helper function to get the path to test data
fn test_data_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test-data");
    path.push("hello-world");
    path.push(filename);
    path
}

#[test]
fn test_elf_ihex_memory_consistency() {
    // Load the ELF file
    let elf_path = test_data_path("hello.elf");
    let elf_content = fs::read(&elf_path)
        .expect("Failed to read hello.elf - make sure to run 'make' in test-data/hello-world/");

    let elf_loader = ElfLoader::default();
    let mut elf_desc = elf_loader
        .load_bytes(Cow::Borrowed(&elf_content), HashMap::new())
        .expect("Failed to load ELF file");

    // Load the Intel HEX file
    let hex_path = test_data_path("hello.hex");
    let hex_content = fs::read(&hex_path)
        .expect("Failed to read hello.hex - make sure to run 'make' in test-data/hello-world/");

    let ihex_loader = IhexLoader;
    let mut ihex_desc = ihex_loader
        .load_bytes(Cow::Borrowed(&hex_content), HashMap::new())
        .expect("Failed to load Intel HEX file");

    // Extract memory regions from both loaders
    let elf_regions = elf_desc.take_memory_regions();
    let ihex_regions = ihex_desc.take_memory_regions();

    // Both loaders should load some regions
    assert!(
        !elf_regions.is_empty(),
        "ELF loader produced no memory regions"
    );
    assert!(
        !ihex_regions.is_empty(),
        "Intel HEX loader produced no memory regions"
    );

    println!("\n=== ELF Loader Regions ===");
    for (i, region) in elf_regions.iter().enumerate() {
        println!(
            "  Region {}: base=0x{:08X}, size={} bytes",
            i,
            region.base(),
            region.size()
        );
    }

    println!("\n=== Intel HEX Loader Regions ===");
    for (i, region) in ihex_regions.iter().enumerate() {
        println!(
            "  Region {}: base=0x{:08X}, size={} bytes",
            i,
            region.base(),
            region.size()
        );
    }

    // For each region in the Intel HEX, verify the content matches the ELF
    for ihex_region in &ihex_regions {
        let ihex_base = ihex_region.base();
        let ihex_size = ihex_region.size();

        // Read the data from Intel HEX region
        let ihex_data = ihex_region
            .read_data(ihex_base, ihex_size)
            .expect("Failed to read Intel HEX region data");

        // Find corresponding data in ELF regions
        let mut found_match = false;
        for elf_region in &elf_regions {
            let elf_base = elf_region.base();
            let elf_size = elf_region.size();

            // Check if the Intel HEX region falls within this ELF region
            if ihex_base >= elf_base && ihex_base < elf_base + elf_size {
                let offset = ihex_base - elf_base;

                // Read the corresponding portion from the ELF region
                let compare_len = std::cmp::min(ihex_size, elf_size - offset);
                let elf_data = elf_region
                    .read_data(ihex_base, compare_len)
                    .expect("Failed to read ELF region data");

                // Compare the data
                assert_eq!(
                    elf_data, ihex_data,
                    "Data mismatch between ELF and Intel HEX at address 0x{ihex_base:08X}"
                );

                found_match = true;
                println!(
                    "✓ Verified {compare_len} bytes at 0x{ihex_base:08X} match between ELF and Intel HEX"
                );
                break;
            }
        }

        assert!(
            found_match,
            "Intel HEX region at 0x{ihex_base:08X} has no corresponding ELF region"
        );
    }

    println!("\n✓ All Intel HEX regions have matching content in ELF");
}

#[test]
fn test_elf_ihex_total_size_consistency() {
    // Load both files
    let elf_path = test_data_path("hello.elf");
    let elf_content = fs::read(&elf_path).expect("Failed to read hello.elf");

    let hex_path = test_data_path("hello.hex");
    let hex_content = fs::read(&hex_path).expect("Failed to read hello.hex");

    let elf_loader = ElfLoader::default();
    let mut elf_desc = elf_loader
        .load_bytes(Cow::Borrowed(&elf_content), HashMap::new())
        .expect("Failed to load ELF file");

    let ihex_loader = IhexLoader;
    let mut ihex_desc = ihex_loader
        .load_bytes(Cow::Borrowed(&hex_content), HashMap::new())
        .expect("Failed to load Intel HEX file");

    // Calculate total sizes
    let elf_regions = elf_desc.take_memory_regions();
    let ihex_regions = ihex_desc.take_memory_regions();

    // Filter out non-loadable sections (like .bss) from ELF for fair comparison
    // Intel HEX only contains initialized data
    let elf_initialized_size: u64 = elf_regions
        .iter()
        .filter(|r| {
            // Only count regions with actual data (not zero-filled)
            // Read a small sample to check if it's all zeros
            let sample_size = std::cmp::min(r.size(), 256);
            if let Ok(data) = r.read_data(r.base(), sample_size) {
                data.iter().any(|&b| b != 0)
            } else {
                false
            }
        })
        .map(|r| r.size())
        .sum();

    let ihex_total_size: u64 = ihex_regions.iter().map(|r| r.size()).sum();

    println!("\n=== Size Comparison ===");
    println!("ELF initialized data size: {elf_initialized_size} bytes");
    println!("Intel HEX total size: {ihex_total_size} bytes");

    // The Intel HEX should contain the same amount of initialized data as the ELF
    // Allow small differences due to alignment/padding
    let size_diff = (elf_initialized_size as i64 - ihex_total_size as i64).abs();
    assert!(
        size_diff <= 16, // Allow up to 16 bytes difference for alignment
        "Size difference between ELF and Intel HEX is too large: {size_diff} bytes"
    );
}

#[test]
fn test_ihex_start_address_extraction() {
    use styx_cpu_type::Arch;

    // Load the Intel HEX file with architecture hint
    let hex_path = test_data_path("hello.hex");
    let hex_content = fs::read(&hex_path).expect("Failed to read hello.hex");

    let ihex_loader = IhexLoader;
    let mut hints = HashMap::new();
    hints.insert(
        Box::from("arch"),
        Box::new(Arch::Arm) as Box<dyn std::any::Any>,
    );

    let mut ihex_desc = ihex_loader
        .load_bytes(Cow::Borrowed(&hex_content), hints)
        .expect("Failed to load Intel HEX file");

    // Check if a start address was set
    let registers = ihex_desc.take_registers();

    if !registers.is_empty() {
        println!("\n=== Intel HEX Start Address ===");
        for (reg, value) in &registers {
            println!("  {reg:?} = 0x{value:08X}");

            // The start address should be in the flash region (0x08000000+)
            if format!("{reg:?}").contains("Pc") {
                assert!(
                    *value >= 0x08000000,
                    "PC start address 0x{value:08X} is outside expected flash region"
                );
            }
        }
    } else {
        println!("\n=== No explicit start address in Intel HEX file ===");
        println!("This is normal for firmware that relies on reset vectors");
    }
}

#[test]
fn test_ihex_elf_entry_point_consistency() {
    use styx_cpu_type::Arch;

    // Load ELF with architecture hint
    let elf_path = test_data_path("hello.elf");
    let elf_content = fs::read(&elf_path).expect("Failed to read hello.elf");

    let elf_loader = ElfLoader::default();
    let mut elf_hints = HashMap::new();
    elf_hints.insert(
        Box::from("arch"),
        Box::new(Arch::Arm) as Box<dyn std::any::Any>,
    );

    let mut elf_desc = elf_loader
        .load_bytes(Cow::Borrowed(&elf_content), elf_hints)
        .expect("Failed to load ELF file");

    // Load Intel HEX with architecture hint
    let hex_path = test_data_path("hello.hex");
    let hex_content = fs::read(&hex_path).expect("Failed to read hello.hex");

    let ihex_loader = IhexLoader;
    let mut ihex_hints = HashMap::new();
    ihex_hints.insert(
        Box::from("arch"),
        Box::new(Arch::Arm) as Box<dyn std::any::Any>,
    );

    let mut ihex_desc = ihex_loader
        .load_bytes(Cow::Borrowed(&hex_content), ihex_hints)
        .expect("Failed to load Intel HEX file");

    // Get entry points from both
    let elf_registers = elf_desc.take_registers();
    let ihex_registers = ihex_desc.take_registers();

    println!("\n=== Entry Point Comparison ===");

    if !elf_registers.is_empty() {
        println!("ELF entry point registers:");
        for (reg, value) in &elf_registers {
            println!("  {reg:?} = 0x{value:08X}");
        }
    } else {
        println!("ELF: No explicit entry point register");
    }

    if !ihex_registers.is_empty() {
        println!("Intel HEX entry point registers:");
        for (reg, value) in &ihex_registers {
            println!("  {reg:?} = 0x{value:08X}");
        }
    } else {
        println!("Intel HEX: No explicit entry point register");
    }

    // If both have entry points, they should match
    if !elf_registers.is_empty() && !ihex_registers.is_empty() {
        // Find PC register in both
        let elf_pc = elf_registers
            .iter()
            .find(|(reg, _)| format!("{reg:?}").contains("Pc"));
        let ihex_pc = ihex_registers
            .iter()
            .find(|(reg, _)| format!("{reg:?}").contains("Pc"));

        if let (Some((_, elf_pc_val)), Some((_, ihex_pc_val))) = (elf_pc, ihex_pc) {
            assert_eq!(
                elf_pc_val, ihex_pc_val,
                "Entry point mismatch: ELF=0x{elf_pc_val:08X}, Intel HEX=0x{ihex_pc_val:08X}"
            );
            println!("✓ Entry points match: 0x{elf_pc_val:08X}");
        }
    }
}
