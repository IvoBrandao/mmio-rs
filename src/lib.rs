#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]
//! Zero-overhead, `no_std` memory-mapped I/O register abstraction.
//!
//! # Quick start
//!
//! ```ignore
//! use mmio_rs::prelude::*;
//!
//! // Implement a critical section for your target once per binary.
//! mmio_rs::impl_critical_section!(
//!     acquire: { /* disable interrupts, return previous state */ 0 },
//!     release: |_state| { /* restore interrupt state */ }
//! );
//!
//! // Define a peripheral's register layout.
//! mmio_rs::register_block! {
//!     pub struct Uart @ 0x4000_1000 {
//!         dr:  u32 => ReadWrite @ 0x00,
//!         sr:  u32 => ReadOnly  @ 0x04,
//!         brr: u32 => ReadWrite @ 0x08,
//!     }
//! }
//!
//! // Use it.
//! Uart::dr().write(b'A' as u32);
//! let status = Uart::sr().read();
//! Uart::brr().write_field(0xFFFF, 0, 0x0683); // set baud divisor
//! ```

mod register;
mod access;
mod block;
mod atomic;
mod irq_guard;

pub use access::{ReadOnly, ReadWrite, WriteOnly};
pub use atomic::AtomicRegister;
pub use block::{DynamicRegisterBlock, RegisterBlock};
pub use irq_guard::{critical_section, IrqGuard};
pub use register::Register;

/// Convenience re-export of the most commonly used types.
///
/// Add `use mmio_rs::prelude::*;` to bring the whole public API into scope.
pub mod prelude {
    pub use crate::access::{ReadOnly, ReadWrite, WriteOnly};
    pub use crate::atomic::AtomicRegister;
    pub use crate::block::{DynamicRegisterBlock, RegisterBlock};
    pub use crate::irq_guard::IrqGuard;
    pub use crate::register::Register;
}
