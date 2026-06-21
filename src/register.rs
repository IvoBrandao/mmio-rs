use core::marker::PhantomData;
use core::ptr;

use crate::access::{Readable, Writeable};

pub struct Register<T: Copy, Access> {
    ptr: *mut T,
    _access: PhantomData<Access>,
}

unsafe impl<T: Copy, Access> Send for Register<T, Access> {}
unsafe impl<T: Copy, Access> Sync for Register<T, Access> {}

impl<T: Copy, Access> Register<T, Access> {
    pub const fn new(addr: usize) -> Self {
        Self {
            ptr: addr as *mut T,
            _access: PhantomData,
        }
    }

    pub const fn uninit() -> Self {
        Self {
            ptr: ptr::null_mut(),
            _access: PhantomData,
        }
    }

    pub fn set_address(&mut self, addr: usize) {
        self.ptr = addr as *mut T;
    }

    pub fn address(&self) -> usize {
        self.ptr as usize
    }

    pub fn is_valid(&self) -> bool {
        !self.ptr.is_null()
    }
}

impl<T: Copy, Access: Readable> Register<T, Access> {
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(self.ptr) }
    }
}

impl<T: Copy, Access: Writeable> Register<T, Access> {
    #[inline(always)]
    pub fn write(&self, value: T) {
        unsafe { ptr::write_volatile(self.ptr, value) }
    }
}

impl<T, Access> Register<T, Access>
where
    T: Copy
        + core::ops::BitOr<Output = T>
        + core::ops::BitAnd<Output = T>
        + core::ops::Not<Output = T>
        + core::ops::BitXor<Output = T>
        + core::ops::Shl<u32, Output = T>
        + core::ops::Shr<u32, Output = T>
        + From<u8>
        + core::ops::BitAnd<Output = T>,
    Access: Readable + Writeable,
{
    #[inline(always)]
    pub fn modify<F: FnOnce(T) -> T>(&self, f: F) {
        let val = self.read();
        self.write(f(val));
    }

    #[inline(always)]
    pub fn set_bit(&self, pos: u32) {
        self.modify(|v| v | (T::from(1u8) << pos));
    }

    #[inline(always)]
    pub fn clear_bit(&self, pos: u32) {
        self.modify(|v| v & !(T::from(1u8) << pos));
    }

    #[inline(always)]
    pub fn get_bit(&self, pos: u32) -> bool
    where
        T: PartialEq,
    {
        let mask = T::from(1u8) << pos;
        let zero = T::from(0u8);
        (self.read() & mask) != zero
    }

    #[inline(always)]
    pub fn or(&self, mask: T) {
        self.modify(|v| v | mask);
    }

    #[inline(always)]
    pub fn and(&self, mask: T) {
        self.modify(|v| v & mask);
    }

    #[inline(always)]
    pub fn xor(&self, mask: T) {
        self.modify(|v| v ^ mask);
    }

    #[inline(always)]
    pub fn write_field(&self, mask: T, shift: u32, value: T) {
        self.modify(|v| (v & !(mask << shift)) | (value << shift));
    }

    #[inline(always)]
    pub fn read_field(&self, mask: T, shift: u32) -> T {
        (self.read() >> shift) & mask
    }
}
