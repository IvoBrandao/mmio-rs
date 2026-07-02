use core::marker::PhantomData;
use core::ptr;

use crate::access::{Readable, Writeable};

/// A typed pointer to a single memory-mapped register.
///
/// `T` is the register width (`u8`, `u16`, `u32`, `u64`).
/// `Access` is one of [`ReadOnly`](crate::ReadOnly), [`WriteOnly`](crate::WriteOnly),
/// or [`ReadWrite`](crate::ReadWrite), enforced at compile time.
pub struct Register<T: Copy, Access> {
    ptr: *mut T,
    _access: PhantomData<Access>,
}

// SAFETY: MMIO register wrappers are safe to transfer between threads when T
// is Send/Sync. The caller is responsible for correct synchronisation; use
// AtomicRegister or IrqGuard for interrupt-safe access.
unsafe impl<T: Copy + Send, Access: Send> Send for Register<T, Access> {}
unsafe impl<T: Copy + Sync, Access: Sync> Sync for Register<T, Access> {}

impl<T: Copy, Access> Register<T, Access> {
    /// Creates a register pointing to `addr`.
    pub const fn new(addr: usize) -> Self {
        Self {
            ptr: addr as *mut T,
            _access: PhantomData,
        }
    }

    /// Creates a register with a null pointer. Call [`set_address`](Self::set_address) before use.
    pub const fn uninit() -> Self {
        Self {
            ptr: ptr::null_mut(),
            _access: PhantomData,
        }
    }

    /// Points this register at a new address.
    pub fn set_address(&mut self, addr: usize) {
        self.ptr = addr as *mut T;
    }

    /// Returns the current address this register points to.
    pub fn address(&self) -> usize {
        self.ptr as usize
    }

    /// Returns `true` if the register has been initialised (address is non-null).
    pub fn is_valid(&self) -> bool {
        !self.ptr.is_null()
    }
}

impl<T: Copy, Access: Readable> Register<T, Access> {
    /// Performs a volatile read of the register.
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(self.ptr) }
    }
}

impl<T: Copy, Access: Writeable> Register<T, Access> {
    /// Performs a volatile write to the register.
    #[inline(always)]
    pub fn write(&self, value: T) {
        unsafe { ptr::write_volatile(self.ptr, value) }
    }
}

// Read-only bitwise accessors — only require Readable.
impl<T, Access> Register<T, Access>
where
    T: Copy
        + core::ops::BitAnd<Output = T>
        + core::ops::Shr<u32, Output = T>,
    Access: Readable,
{
    /// Reads a bit-field: shifts right by `shift` then masks with `mask`.
    #[inline(always)]
    pub fn read_field(&self, mask: T, shift: u32) -> T {
        (self.read() >> shift) & mask
    }

    /// Returns `true` if the bit at position `pos` is set.
    #[inline(always)]
    pub fn get_bit(&self, pos: u32) -> bool
    where
        T: core::ops::Shl<u32, Output = T> + From<u8> + PartialEq,
    {
        let mask = T::from(1u8) << pos;
        (self.read() & mask) != T::from(0u8)
    }
}

// Read-modify-write operations — require both Readable and Writeable.
impl<T, Access> Register<T, Access>
where
    T: Copy
        + core::ops::BitOr<Output = T>
        + core::ops::BitAnd<Output = T>
        + core::ops::Not<Output = T>
        + core::ops::BitXor<Output = T>
        + core::ops::Shl<u32, Output = T>
        + From<u8>,
    Access: Readable + Writeable,
{
    /// Reads the register, applies `f` to the value, and writes the result back.
    #[inline(always)]
    pub fn modify<F: FnOnce(T) -> T>(&self, f: F) {
        let val = self.read();
        self.write(f(val));
    }

    /// Sets bit `pos` (read-modify-write).
    #[inline(always)]
    pub fn set_bit(&self, pos: u32) {
        self.modify(|v| v | (T::from(1u8) << pos));
    }

    /// Clears bit `pos` (read-modify-write).
    #[inline(always)]
    pub fn clear_bit(&self, pos: u32) {
        self.modify(|v| v & !(T::from(1u8) << pos));
    }

    /// ORs the register value with `mask` (sets every bit that is 1 in `mask`).
    #[inline(always)]
    pub fn set_bits(&self, mask: T) {
        self.modify(|v| v | mask);
    }

    /// ANDs the register value with `mask` (clears every bit that is 0 in `mask`).
    #[inline(always)]
    pub fn and_bits(&self, mask: T) {
        self.modify(|v| v & mask);
    }

    /// XORs the register value with `mask` (toggles every bit that is 1 in `mask`).
    #[inline(always)]
    pub fn toggle_bits(&self, mask: T) {
        self.modify(|v| v ^ mask);
    }

    /// Writes `value` into the bit-field defined by `mask` at `shift`, preserving all other bits.
    /// `value` is automatically clamped to `mask` width to prevent bleeding into adjacent fields.
    #[inline(always)]
    pub fn write_field(&self, mask: T, shift: u32, value: T) {
        // Mask the value first so bits above the field width cannot bleed into
        // neighbouring fields (e.g. write_field(0xF, 4, 0xFF) must not corrupt
        // bits [7:8] of the register).
        self.modify(|v| (v & !(mask << shift)) | ((value & mask) << shift));
    }
}
