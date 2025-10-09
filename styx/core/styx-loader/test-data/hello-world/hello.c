// SPDX-License-Identifier: BSD-2-Clause
// Simple hello world program for testing Intel hex loader
// This is compiled for bare metal ARM to create a minimal binary

// Define a simple string in memory
const char hello_msg[] = "Hello, World!\n";

// Entry point at a specific address (typically reset vector location)
void _start(void) __attribute__((naked, section(".text.start")));

void _start(void) {
    // Simple function that just references our string
    // In a real embedded system, this might write to a UART
    volatile const char *msg = hello_msg;

    // Simple loop to prevent optimization
    for (int i = 0; i < 14; i++) {
        volatile char c = msg[i];
        (void)c; // Suppress unused warning
    }

    // Infinite loop (common in embedded systems)
    while (1) {
        __asm__("nop");
    }
}

// Add some initialized data in different sections
const unsigned int magic_number __attribute__((section(".rodata"))) = 0xDEADBEEF;
const unsigned int version_info __attribute__((section(".rodata"))) = 0x01020304;

// Add a data section with some values
unsigned int data_values[] __attribute__((section(".data"))) = {
    0x11111111,
    0x22222222,
    0x33333333,
    0x44444444
};
