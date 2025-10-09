/* SPDX-License-Identifier: BSD-2-Clause */
/* Linker script for ARM bare metal loader hello world test */

MEMORY
{
    /* Typical ARM Cortex-M memory layout */
    FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 64K
    RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 16K
}

ENTRY(_start)

SECTIONS
{
    /* Interrupt vector table (first thing in flash) */
    .isr_vector :
    {
        . = ALIGN(4);
        KEEP(*(.isr_vector))
        . = ALIGN(4);
    } > FLASH

    /* Program code */
    .text :
    {
        . = ALIGN(4);
        *(.text.start)  /* Ensure _start is placed first */
        *(.text)
        *(.text*)
        . = ALIGN(4);
    } > FLASH

    /* Read-only data */
    .rodata :
    {
        . = ALIGN(4);
        *(.rodata)
        *(.rodata*)
        . = ALIGN(4);
    } > FLASH

    /* section ".deadbeef" to be a large offset from .data, */
    /* this section is in FLASH */
    .deadbeef ADDR(.rodata) + SIZEOF(.rodata) + 4096 :
        AT(ADDR(.rodata) + SIZEOF(.rodata) + 4096)
    {
        . = ALIGN(4);
        KEEP(*(.deadbeef))
        KEEP(*(.deadbeef.*))
        . = ALIGN(4);
    } > FLASH

    /* make another section  ".counter" to have known data, */
    /* this section is in RAM */
    .counter ADDR(.deadbeef) + SIZEOF(.deadbeef) + 4096  :
        AT(ADDR(.deadbeef) + SIZEOF(.deadbeef) + 4096)
    {
        . = ALIGN(4);
        KEEP(*(.counter))
        KEEP(*(.counter.*))
        . = ALIGN(4);
    } > FLASH

    /* Initialized data (will be copied from flash to RAM) */
    .data :
    {
        . = ALIGN(4);
        _sdata = .;        /* Start of data */
        *(.data)
        *(.data*)
        . = ALIGN(4);
        _edata = .;        /* End of data */
    } > RAM AT> FLASH

    /* Uninitialized data */
    .bss :
    {
        . = ALIGN(4);
        _sbss = .;         /* Start of bss */
        *(.bss)
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = .;         /* End of bss */
    } > RAM

    /* Remove unused sections */
    /DISCARD/ :
    {
        *(.comment)
        *(.ARM.attributes)
        *(.debug_*)
    }
}
