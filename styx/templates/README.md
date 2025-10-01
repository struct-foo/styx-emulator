# Styx Component Templates Quick Start

This guide will walk you through creating your first Styx component using cargo-generate templates.

## Installation

```bash
# Install cargo-generate if you haven't already
cargo install cargo-generate
```

## Creating Your First Component

### 1. Choose Your Component Type

Decide what type of component you need:

- **Processor**: Full microcontroller/SoC implementation
- **Event Controller**: Interrupt controller
- **Peripheral**: Hardware device (UART, SPI, timer, etc.)
- **Plugin**: Modular processor extension

### 2. Generate the Component

From the Styx repository root:

```bash
cargo generate --path styx/templates
```

Answer the prompts:

```
? What type of component are you creating? peripheral
? Is this an in-tree component (part of the Styx workspace)? true
? What is the name of your component? my-timer
? Brief description of your component: A simple timer peripheral
? Author name: Jane Doe
? Author email: jane@example.com
```

### 3. Locate Your New Component

The generator creates a new crate. Move it to the appropriate location:

```bash
# For a peripheral
mv styx-my-timer styx/peripherals/

# For a processor
mv styx-my-timer-processor styx/processors/<arch>/

# For an event controller
mv styx-my-timer styx/event-controllers/<arch>/

# For a plugin
mv styx-my-timer styx/plugins/
```

### 4. Add to Workspace

Edit the root `Cargo.toml` to include your component:

```toml
[workspace]
members = [
    # ... existing members
    "styx/peripherals/styx-my-timer",
]
```

### 5. Implement Your Component

Open `src/lib.rs` and look for `TODO` comments:

```rust
// Example for a peripheral
impl Peripheral for MyTimer {
    fn init(&mut self, proc: &mut BuildingProcessor) -> Result<(), UnknownError> {
        // TODO: Register memory hooks for peripheral registers
        // Register a hook for the timer control register at 0x4000_0000
        proc.core.cpu.add_hook(StyxHook::memory_write(
            0x4000_0000,
            |handle, addr, size, value| {
                // Handle writes to control register
                Ok(())
            }
        ))?;

        Ok(())
    }

    fn tick(
        &mut self,
        cpu: &mut dyn CpuBackend,
        mmu: &mut Mmu,
        event_controller: &mut dyn EventControllerImpl,
        delta: &Delta,
    ) -> Result<(), UnknownError> {
        // Update timer state based on elapsed time/instructions
        self.counter += delta.instructions;

        // Trigger interrupt if timer expired
        if self.counter >= self.target {
            event_controller.latch(self.irq_number)?;
        }

        Ok(())
    }

    // Implement other required methods...
}
```

### 6. Build and Test

```bash
# Build your component
cargo build -p styx-my-timer

# Run tests
cargo test -p styx-my-timer

# Build entire workspace
cargo build
```

## Component Lifecycle

Understanding the lifecycle of each component type is essential for proper implementation:

### Processor Lifecycle

```
ProcessorBuilder::with_builder(MyProcessorBuilder)
    └─> ProcessorImpl::build()  [Create CPU, MMU, EventController, Peripherals]
        └─> ProcessorImpl::init()  [Initialize registers, hooks, state]
            └─> Processor ready for execution
```

### Event Controller Lifecycle

```
EventController::init()
    └─> EventController::latch(irq)  [Queue interrupt]
        └─> EventController::next()  [Select and execute interrupt]
            └─> EventController::finish_interrupt()  [Cleanup]
```

### Peripheral Lifecycle

```
Peripheral::init()  [Register hooks, setup state]
    └─> on_processor_start()  [Startup behavior]
        └─> tick()  [Periodic updates during execution]
            └─> post_event_hook()  [Handle interrupt completion]
                └─> on_processor_stop()  [Cleanup]
```

### Plugin Lifecycle

```
UninitPlugin::init()  [Initialize, register hooks]
    └─> Plugin::plugins_initialized_hook()  [Cross-plugin setup]
        └─> Plugin::on_processor_start()
            └─> Plugin::tick()  [Runtime behavior]
                └─> Plugin::on_processor_stop()
```

## Common Patterns

### Registering Memory Hooks

```rust
// Read hook
proc.core.cpu.add_hook(StyxHook::memory_read(
    address,
    |handle, addr, size| {
        let value = /* compute value */;
        handle.mmu.data().write(addr).bytes(&value)?;
        Ok(())
    }
))?;

// Write hook
proc.core.cpu.add_hook(StyxHook::memory_write(
    address,
    |handle, addr, size, value| {
        // Process the written value
        Ok(())
    }
))?;
```

### Triggering Interrupts

```rust
// From a peripheral's tick() method
impl Peripheral for MyPeripheral {
    fn tick(
        &mut self,
        cpu: &mut dyn CpuBackend,
        mmu: &mut Mmu,
        event_controller: &mut dyn EventControllerImpl,
        delta: &Delta,
    ) -> Result<(), UnknownError> {
        if self.should_interrupt() {
            event_controller.latch(self.irq_number)?;
        }
        Ok(())
    }
}
```

### Accessing Memory

```rust
// Read from memory
let value: u32 = handle.mmu.data().read(address).u32()?;

// Write to memory
handle.mmu.data().write(address).u32(0x1234_5678)?;
```

## Template Validation and CI

The templates are automatically validated in CI to ensure they work correctly.

### Running Validation Locally

You can test the templates locally using the provided script:

```bash
# From repository root
./styx/templates/scripts/test-templates-local.sh
```

This script will:
- Generate all template types in `/tmp/styx-template-tests-<timestamp>`
- Build each one with proper styx-core dependencies
- Verify trait implementations
- Report success or failure with detailed, colored output

### CI Workflow

The GitHub Actions workflow (`.github/workflows/template-validation.yml`) runs the same validation automatically on:
- Pull requests that modify template files
- Pushes to main that touch templates
- Manual workflow dispatch

The CI validates:
1. All 4 template types (processor, event-controller, peripheral, plugin)
2. Cargo.toml dependency paths are correct
3. Generated code compiles successfully
4. Expected trait implementations exist

## Debugging Tips

### Enable Logging

```bash
# Set log level
export RUST_LOG=trace

# Or for specific modules
export RUST_LOG=styx_my_timer=trace

# Run with logging
cargo run
```

### Add Tracing

```rust
use tracing::{trace, debug, info, warn, error};

fn my_function() {
    trace!("Detailed trace information");
    debug!("Debug information: value = {}", value);
    info!("Important milestone reached");
    warn!("Potential issue detected");
    error!("Error occurred: {}", err);
}
```

## Examples to Study

- **Simple Peripheral**: `styx/peripherals/styx-uart/src/lib.rs`
- **Complex Processor**: `styx/processors/ppc/styx-ppc4xx-processor/src/lib.rs`
- **Event Controller**: `styx/event-controllers/arm/styx-nvic/src/lib.rs`
- **Plugin**: `styx/plugins/styx-gdbserver/src/lib.rs`

## Resources

- [Styx Core Documentation](../../core/README.md)
- [cargo-generate Documentation](https://cargo-generate.github.io/cargo-generate/)
- [Styx Examples](../../examples/)

Happy coding! 🦀
