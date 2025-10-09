// SPDX-License-Identifier: BSD-2-Clause
// Simple hello world program for testing Intel hex loader
// This is compiled for bare metal ARM to create a minimal binary

// Define a simple string in memory
const char hello_msg[] = "Hello, World!\n";

// Entry point
void _start(void) __attribute__((naked, section(".text.start")));

void _start(void) {
    // Simple function that just references our string
    volatile const char *msg = hello_msg;

    // Simple loop to prevent optimization
    for (int i = 0; i < 14; i++) {
        volatile char c = msg[i];

        // Suppress unused warning
        (void)c;
    }

    // Infinite loop (common in embedded systems)
    while (1) {
        __asm__("nop");
    }
}

// Add some initialized data in different sections
const unsigned int magic_number __attribute__((section(".deadbeef"))) = 0xDEADBEEF;
const unsigned int version_info __attribute__((section(".counter"))) = 0x01020304;
