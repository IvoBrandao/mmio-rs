use core::marker::PhantomData;

use crate::access::ReadWrite;
use crate::register::Register;

/// A fixed-size array of `N` registers sharing a base address, addressed by index.
///
/// All registers are typed as `ReadWrite`. For per-field access control use the
/// [`register_block!`](crate::register_block) macro instead.
pub struct RegisterBlock<const N: usize> {
    base: usize,
    offsets: [usize; N],
}

impl<const N: usize> RegisterBlock<N> {
    /// Creates a block at `base` with the given byte `offsets`.
    pub const fn new(base: usize, offsets: [usize; N]) -> Self {
        Self { base, offsets }
    }

    /// Returns the base address.
    pub fn base(&self) -> usize {
        self.base
    }

    /// Updates the base address.
    pub fn set_base(&mut self, base: usize) {
        self.base = base;
    }

    /// Returns `true` if the base address is non-zero.
    pub fn is_valid(&self) -> bool {
        self.base != 0
    }

    /// Returns the register at `index`. Panics if `index >= N`.
    pub fn reg<T: Copy>(&self, index: usize) -> Register<T, ReadWrite> {
        assert!(index < N, "register index {index} out of bounds (block has {N} registers)");
        Register::new(self.base + self.offsets[index])
    }

    /// Returns the register at `index`, or `None` if out of bounds.
    pub fn reg_checked<T: Copy>(&self, index: usize) -> Option<Register<T, ReadWrite>> {
        self.offsets.get(index).map(|&off| Register::new(self.base + off))
    }
}

/// Like [`RegisterBlock`] but the register type `T` and access marker `Access` are
/// fixed at construction time, and the base address can be set after creation.
pub struct DynamicRegisterBlock<T: Copy, Access, const N: usize> {
    base: usize,
    offsets: [usize; N],
    _phantom: PhantomData<(T, Access)>,
}

impl<T: Copy, Access, const N: usize> DynamicRegisterBlock<T, Access, N> {
    /// Creates a block at `base` with the given byte `offsets`.
    pub const fn new(base: usize, offsets: [usize; N]) -> Self {
        Self {
            base,
            offsets,
            _phantom: PhantomData,
        }
    }

    /// Creates a block with a zero base address. Call [`set_base_address`](Self::set_base_address) before use.
    pub const fn uninit(offsets: [usize; N]) -> Self {
        Self {
            base: 0,
            offsets,
            _phantom: PhantomData,
        }
    }

    /// Sets the base address.
    pub fn set_base_address(&mut self, base: usize) {
        self.base = base;
    }

    /// Returns the base address.
    pub fn base_address(&self) -> usize {
        self.base
    }

    /// Returns `true` if the base address is non-zero.
    pub fn is_valid(&self) -> bool {
        self.base != 0
    }

    /// Returns the register at `index`. Panics if `index >= N`.
    pub fn get(&self, index: usize) -> Register<T, Access> {
        assert!(index < N, "register index {index} out of bounds (block has {N} registers)");
        Register::new(self.base + self.offsets[index])
    }

    /// Returns the register at `index`, or `None` if out of bounds.
    pub fn get_checked(&self, index: usize) -> Option<Register<T, Access>> {
        self.offsets.get(index).map(|&off| Register::new(self.base + off))
    }
}

/// Define a register block with named fields and a static base address.
///
/// Two field syntaxes are supported within each struct:
///
/// **Simple** (defaults to `u32` / `ReadWrite`):
/// ```ignore
/// register_block! {
///     pub struct SysTick @ 0xE000_E010 {
///         csr: 0x00,
///         rvr: 0x04,
///     }
/// }
/// let csr = SysTick::csr();
/// ```
///
/// **Extended** (explicit type and access per field via `Type => Access @ offset`):
/// ```ignore
/// register_block! {
///     pub struct Uart @ 0x4000_1000 {
///         dr:  u32 => ReadWrite @ 0x00,
///         sr:  u32 => ReadOnly  @ 0x04,
///         brr: u16 => ReadWrite @ 0x08,
///     }
/// }
/// ```
#[macro_export]
macro_rules! register_block {
    // === Static block @ compile-time base address ===

    // Simple syntax: field: offset  →  Register<u32, ReadWrite>
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

    // Extended syntax: field: Type => Access @ offset  →  Register<Type, Access>
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident @ $base:literal {
            $( $field:ident : $ty:ty => $access:ident @ $offset:literal ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $name;

        impl $name {
            $(
                #[inline(always)]
                $vis fn $field() -> $crate::Register<$ty, $crate::$access> {
                    $crate::Register::new($base + $offset)
                }
            )*
        }
    };

    // === Dynamic block (runtime base address) ===

    // Simple syntax: field: offset  →  Register<u32, ReadWrite>
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

    // Extended syntax: field: Type => Access @ offset  →  Register<Type, Access>
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $( $field:ident : $ty:ty => $access:ident @ $offset:literal ),* $(,)?
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
                $vis fn $field(&self) -> $crate::Register<$ty, $crate::$access> {
                    $crate::Register::new(self.base + $offset)
                }
            )*
        }
    };
}
