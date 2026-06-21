# mmio-rs

Zero-overhead, `no_std`, zero-dependency Rust crate for type-safe memory-mapped I/O register access.

[![Crates.io](https://img.shields.io/crates/v/mmio-rs.svg)](https://crates.io/crates/mmio-rs)

---

## Why

Embedded drivers need to read and write hardware registers at specific memory addresses. Raw `*mut u32` casts are unsafe, untyped, and untestable. This crate provides:

- Compile-time access enforcement (read-only, write-only, read-write)
- Named register blocks with offset calculation
- Atomic read-modify-write in user-defined critical sections
- Architecture-agnostic design (ARM, RISC-V, Xtensa, or host tests)
- Testable on host — registers can point to stack memory

## Quick Start

```rust
use mmio_rs::prelude::*;

// Required once per project — tells the library how to disable interrupts
mmio_rs::impl_critical_section!(
    acquire: { 0 },       // no-op for host
    release: |_state| {}
);

// Define a peripheral
mmio_rs::register_block! {
    pub struct GpioA @ 0x4800_0000 {
        moder: 0x00,
        odr:   0x14,
        bsrr:  0x18,
    }
}

// Use it
GpioA::odr().set_bit(5);         // PA5 high
GpioA::moder().write_field(0x3, 10, 0x1); // PA5 output mode
```

## Features

| Feature | Description |
|---------|-------------|
| **Register sizes** | `u8`, `u16`, `u32`, `u64` |
| **Access policies** | `ReadOnly`, `WriteOnly`, `ReadWrite` — compile-time enforced |
| **Bit operations** | `set_bit()`, `clear_bit()`, `get_bit()` |
| **Field access** | `write_field(mask, shift, val)`, `read_field(mask, shift)` |
| **Compound ops** | `or()`, `and()`, `xor()`, `modify()` |
| **Register blocks** | `register_block!` macro — static or dynamic base |
| **Three init modes** | Static `@`, constructor `.new(addr)`, late `.uninit()` + `.set_base_address()` |
| **Atomic RMW** | `AtomicRegister` with user-provided critical section |
| **Zero dependencies** | Only `core` — no alloc, no std, no external crates |
| **Architecture-agnostic** | User provides interrupt disable/restore via `impl_critical_section!` |

## Critical Section

The library does not know your target architecture. You provide the critical section once:

```rust
// ARM Cortex-M
mmio_rs::impl_critical_section!(
    acquire: {
        let primask: u32;
        core::arch::asm!("MRS {}, PRIMASK", out(reg) primask);
        core::arch::asm!("CPSID i", options(nomem, nostack));
        primask
    },
    release: |state| {
        core::arch::asm!("MSR PRIMASK, {}", in(reg) state, options(nomem, nostack));
    }
);
```

If you forget, you get a linker error — not a runtime failure.

## Examples

```sh
cargo run --example basic_register
cargo run --example register_block
cargo run --example atomic_operations
cargo run --example peripheral_driver
```

## Project Structure

```
mmio-rs/
├── src/
│   ├── lib.rs          Re-exports and prelude
│   ├── register.rs     Register<T, Access> — core abstraction
│   ├── access.rs       ReadOnly, WriteOnly, ReadWrite types
│   ├── block.rs        RegisterBlock, DynamicRegisterBlock, register_block! macro
│   ├── atomic.rs       AtomicRegister — interrupt-safe RMW
│   └── irq_guard.rs    IrqGuard, critical_section, impl_critical_section!
├── tests/
│   └── register_tests.rs   26 integration tests
├── examples/
│   ├── basic_register.rs   Single register operations
│   ├── register_block.rs   All three initialization modes
│   ├── atomic_operations.rs Interrupt-safe operations
│   └── peripheral_driver.rs Complete driver pattern
├── docs/
│   ├── architecture.md  Module diagrams and design decisions
│   └── usage.md         Step-by-step guide with code examples
├── Cargo.toml
└── README.md
```

## Documentation

- [Architecture](docs/architecture.md) — module structure, type system, critical section design
- [Usage Guide](docs/usage.md) — setup, register blocks, driver patterns, testing

## License

MIT
