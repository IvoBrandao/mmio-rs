# Architecture

## Overview

`mmio-rs` is a zero-overhead, `no_std`, zero-dependency Rust crate for type-safe access to memory-mapped hardware registers. It compiles to the same instructions as raw volatile pointer access, with compile-time safety guarantees.

## Module Structure

```mermaid
graph TD
    subgraph "User Project"
        APP[Driver / Application]
        CS[impl_critical_section!<br/>User-provided]
    end

    subgraph "mmio-rs Crate"
        LIB[lib.rs — re-exports]
        REG[register.rs<br/>Register T, Access]
        ACC[access.rs<br/>ReadOnly, WriteOnly, ReadWrite]
        BLK[block.rs<br/>RegisterBlock, DynamicRegisterBlock<br/>register_block! macro]
        ATM[atomic.rs<br/>AtomicRegister]
        IRQ[irq_guard.rs<br/>IrqGuard, critical_section]
    end

    subgraph "Hardware"
        HW[Memory-Mapped Registers]
    end

    APP --> LIB
    CS -->|links| IRQ

    LIB --> REG
    LIB --> ACC
    LIB --> BLK
    LIB --> ATM
    LIB --> IRQ

    REG --> ACC
    ATM --> REG
    ATM --> IRQ
    BLK --> REG

    REG -->|"ptr::read/write_volatile"| HW
```

## Type System Enforcement

```mermaid
graph LR
    subgraph "Access Traits"
        Readable
        Writeable
    end

    subgraph "Access Types"
        RO[ReadOnly]
        WO[WriteOnly]
        RW[ReadWrite]
    end

    RO -->|impl| Readable
    WO -->|impl| Writeable
    RW -->|impl| Readable
    RW -->|impl| Writeable

    subgraph "Methods enabled by trait bounds"
        R["read(), get_bit()"]
        W["write()"]
        RMW["modify(), set_bit(), clear_bit(),<br/>or(), and(), xor(), write_field()"]
    end

    Readable -.->|"where Access: Readable"| R
    Writeable -.->|"where Access: Writeable"| W
    Readable -.->|"where Access: Readable + Writeable"| RMW
    Writeable -.->|"where Access: Readable + Writeable"| RMW
```

Attempting to call `write()` on a `Register<u32, ReadOnly>` produces a compile error — no runtime cost.

## Critical Section Architecture

```mermaid
sequenceDiagram
    participant User as User Code
    participant Atomic as AtomicRegister
    participant IRQ as irq_guard.rs
    participant Extern as User-provided impl
    participant HW as Hardware

    User->>Atomic: set_bits(mask)
    Atomic->>IRQ: critical_section(closure)
    IRQ->>Extern: _mmio_critical_section_acquire()
    Extern-->>IRQ: saved state
    Note right of Extern: Interrupts disabled

    IRQ->>Atomic: execute closure
    Atomic->>HW: read_volatile
    HW-->>Atomic: value
    Atomic->>Atomic: value | mask
    Atomic->>HW: write_volatile

    IRQ->>Extern: _mmio_critical_section_release(state)
    Note right of Extern: Interrupts restored
```

The library declares `extern "Rust"` functions. The user links them via `impl_critical_section!`. This keeps the crate architecture-agnostic — it works on ARM, RISC-V, Xtensa, or even host tests.

## Register Block Initialization Modes

```mermaid
stateDiagram-v2
    [*] --> Static: Base known at compile time
    [*] --> Constructor: Base known at runtime
    [*] --> Late: Base discovered later

    Static --> Ready: register_block! with @ addr
    Constructor --> Ready: new(addr)
    Late --> Uninitialized: uninit()
    Uninitialized --> Ready: set_base_address(addr)

    Ready --> Operating: read / write / modify
```

## Zero-Cost Guarantee

Every register operation compiles to a single volatile load or store instruction. The type system exists only at compile time:

```mermaid
graph LR
    subgraph "Rust Source"
        A["reg.set_bit(3)"]
    end

    subgraph "After Optimization"
        B["ldr r0, [r1]<br/>orr r0, #8<br/>str r0, [r1]"]
    end

    A -->|"rustc -O"| B
```

No vtables, no heap, no indirection, no branches for access checks.

## Design Decisions

| Decision | Rationale |
|----------|-----------|
| `extern "Rust"` for critical section | Zero deps, user configures per-platform, linker error if missing |
| Trait bounds for access | Compile-time enforcement, zero runtime cost |
| `#[inline(always)]` everywhere | Ensures optimizer sees through all abstractions |
| `*mut T` pointer internally | Allows null state for two-phase init |
| `register_block!` macro | Named fields ergonomics without proc-macro dependency |
| `no_std` only | Embedded-first, works everywhere including hosted |
| `Send + Sync` on Register | Registers are memory addresses, safe to share references |
