/* SPDX-License-Identifier: BSD-2-Clause */
/* Linker script for ARM bare metal hello world test */

MEMORY
{
    /* Typical ARM Cortex-M memory layout */
    /* Flash starts at 0x08000000 (common for STM32) */
    FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 64K
    /* RAM starts at 0x20000000 */
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
