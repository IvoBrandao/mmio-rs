#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

mod register;
mod access;
mod block;
mod atomic;
mod irq_guard;

pub use access::{ReadOnly, ReadWrite, WriteOnly};
pub use atomic::AtomicRegister;
pub use block::{DynamicRegisterBlock, RegisterBlock};
pub use irq_guard::IrqGuard;
pub use register::Register;

pub mod prelude {
    pub use crate::access::{ReadOnly, ReadWrite, WriteOnly};
    pub use crate::atomic::AtomicRegister;
    pub use crate::block::{DynamicRegisterBlock, RegisterBlock};
    pub use crate::irq_guard::IrqGuard;
    pub use crate::register::Register;
}
