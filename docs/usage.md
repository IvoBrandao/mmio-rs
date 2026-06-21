# Usage Guide

## Setup

Add to your `Cargo.toml`:

```toml
[dependencies]
mmio-rs = { path = "../libs/mmio-rs" }  # or from crates.io
```

## Step 1: Provide Critical Section

Every project using `mmio-rs` must define the critical section implementation once. This tells the library how to disable/restore interrupts on your target.

### ARM Cortex-M

```rust
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

### RISC-V

```rust
mmio_rs::impl_critical_section!(
    acquire: {
        let mstatus: u32;
        core::arch::asm!("csrrci {}, mstatus, 0x8", out(reg) mstatus);
        mstatus & 0x8
    },
    release: |state| {
        if state != 0 {
            core::arch::asm!("csrsi mstatus, 0x8", options(nomem, nostack));
        }
    }
);
```

### Host / Tests (no-op)

```rust
mmio_rs::impl_critical_section!(
    acquire: { 0 },
    release: |_state| {}
);
```

## Step 2: Define Register Blocks

### Static Block (compile-time base address)

For peripherals at fixed addresses (GPIO, SysTick, NVIC):

```rust
mmio_rs::register_block! {
    pub struct GpioA @ 0x4800_0000 {
        moder:  0x00,
        otyper: 0x04,
        ospeedr: 0x08,
        pupdr:  0x0C,
        idr:    0x10,
        odr:    0x14,
        bsrr:   0x18,
    }
}

// Usage — zero overhead, address is a constant
GpioA::odr().set_bit(5);  // PA5 high
```

### Dynamic Block (runtime base address)

For peripherals with multiple instances (UART1, UART2, SPI1, SPI2):

```rust
mmio_rs::register_block! {
    pub struct UsartRegs {
        sr:  0x00,
        dr:  0x04,
        brr: 0x08,
        cr1: 0x0C,
        cr2: 0x10,
        cr3: 0x14,
    }
}

// Mode 2: construct with address
let uart1 = UsartRegs::new(0x4001_1000);
let uart2 = UsartRegs::new(0x4000_4400);

// Mode 3: late init
let mut uart3 = UsartRegs::uninit();
// ... later ...
uart3.set_base_address(0x4000_4800);
```

## Step 3: Use Registers

### Basic Operations

```rust
let cr1 = uart1.cr1();

// Write full value
cr1.write(0x0000_200C);

// Read
let status = uart1.sr().read();

// Single bit
cr1.set_bit(13);    // set UE (USART enable)
cr1.clear_bit(13);  // clear UE
let enabled = cr1.get_bit(13);

// Compound
cr1.or(0x0C);   // set TE + RE
cr1.and(!0x04); // clear RE

// Read-modify-write with closure
cr1.modify(|v| (v & !0xF000) | 0x3000);
```

### Field Access

```rust
// Write a multi-bit field: bits [15:12] = 0x5
cr1.write_field(0xF, 12, 0x5);

// Read a multi-bit field
let baud_mantissa = uart1.brr().read_field(0xFFF, 4);
```

### Atomic Operations

For registers shared between ISR and main loop:

```rust
let reg = GpioA::odr();
let atomic = AtomicRegister::new(&reg);

atomic.set_bits(1 << 5);    // PA5 high (interrupt-safe)
atomic.clear_bits(1 << 5);  // PA5 low
atomic.toggle_bits(1 << 5); // toggle

atomic.modify(|v| (v & !0xFF) | 0x42); // custom RMW
```

## Writing a Peripheral Driver

```rust
use mmio_rs::prelude::*;

mmio_rs::register_block! {
    struct TimRegs {
        cr1:  0x00,
        dier: 0x0C,
        sr:   0x10,
        cnt:  0x24,
        psc:  0x28,
        arr:  0x2C,
    }
}

pub struct Timer {
    regs: TimRegs,
}

impl Timer {
    pub fn new(base: usize) -> Self {
        Self { regs: TimRegs::new(base) }
    }

    pub fn configure(&self, prescaler: u16, period: u32) {
        self.regs.psc().write(prescaler as u32);
        self.regs.arr().write(period);
        self.regs.cr1().set_bit(0); // CEN
    }

    pub fn enable_interrupt(&self) {
        self.regs.dier().set_bit(0); // UIE
    }

    pub fn clear_pending(&self) {
        self.regs.sr().clear_bit(0); // UIF
    }

    pub fn count(&self) -> u32 {
        self.regs.cnt().read()
    }
}
```

## Testing on Host

Registers can point to stack memory — no hardware needed:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mmio_rs::impl_critical_section!(
        acquire: { 0 },
        release: |_state| {}
    );

    #[test]
    fn timer_configures_correctly() {
        let mut hw: [u32; 12] = [0; 12];
        let tim = Timer::new(hw.as_mut_ptr() as usize);

        tim.configure(79, 999);
        assert_eq!(hw[10], 79);  // PSC at offset 0x28 = index 10
        assert_eq!(hw[11], 999); // ARR at offset 0x2C = index 11
    }
}
```
