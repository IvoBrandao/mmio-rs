use core::marker::PhantomData;

use crate::access::ReadWrite;
use crate::register::Register;

pub struct RegisterBlock<const N: usize> {
    base: usize,
    offsets: [usize; N],
}

impl<const N: usize> RegisterBlock<N> {
    pub const fn new(base: usize, offsets: [usize; N]) -> Self {
        Self { base, offsets }
    }

    pub fn base(&self) -> usize {
        self.base
    }

    pub fn set_base(&mut self, base: usize) {
        self.base = base;
    }

    pub fn is_valid(&self) -> bool {
        self.base != 0
    }

    pub fn reg<T: Copy>(&self, index: usize) -> Register<T, ReadWrite> {
        Register::new(self.base + self.offsets[index])
    }
}

pub struct DynamicRegisterBlock<T: Copy, Access, const N: usize> {
    base: usize,
    offsets: [usize; N],
    _phantom: PhantomData<(T, Access)>,
}

impl<T: Copy, Access, const N: usize> DynamicRegisterBlock<T, Access, N> {
    pub const fn new(base: usize, offsets: [usize; N]) -> Self {
        Self {
            base,
            offsets,
            _phantom: PhantomData,
        }
    }

    pub const fn uninit(offsets: [usize; N]) -> Self {
        Self {
            base: 0,
            offsets,
            _phantom: PhantomData,
        }
    }

    pub fn set_base_address(&mut self, base: usize) {
        self.base = base;
    }

    pub fn base_address(&self) -> usize {
        self.base
    }

    pub fn is_valid(&self) -> bool {
        self.base != 0
    }

    pub fn get(&self, index: usize) -> Register<T, Access> {
        Register::new(self.base + self.offsets[index])
    }
}

/// Define a register block with named fields and a static base address.
///
/// ```ignore
/// register_block! {
///     pub struct SysTick @ 0xE000_E010 {
///         csr: 0x00,
///         rvr: 0x04,
///         cvr: 0x08,
///     }
/// }
/// let csr = SysTick::csr();
/// ```
#[macro_export]
macro_rules! register_block {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident @ $base:literal {
            $( $field:ident : $offset:literal ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $name;

        impl $name {
            $(
                #[inline(always)]
                $vis fn $field() -> $crate::Register<u32, $crate::ReadWrite> {
                    $crate::Register::new($base + $offset)
                }
            )*
        }
    };

    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $( $field:ident : $offset:literal ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $name {
            base: usize,
        }

        impl $name {
            #[inline(always)]
            $vis const fn new(base: usize) -> Self {
                Self { base }
            }

            #[inline(always)]
            $vis const fn uninit() -> Self {
                Self { base: 0 }
            }

            #[inline(always)]
            $vis fn set_base_address(&mut self, base: usize) {
                self.base = base;
            }

            #[inline(always)]
            $vis fn is_valid(&self) -> bool {
                self.base != 0
            }

            $(
                #[inline(always)]
                $vis fn $field(&self) -> $crate::Register<u32, $crate::ReadWrite> {
                    $crate::Register::new(self.base + $offset)
                }
            )*
        }
    };
}
