extern "Rust" {
    fn _mmio_critical_section_acquire() -> u32;
    fn _mmio_critical_section_release(state: u32);
}

pub struct IrqGuard {
    state: u32,
}

impl IrqGuard {
    #[inline(always)]
    pub fn acquire() -> Self {
        let state = unsafe { _mmio_critical_section_acquire() };
        Self { state }
    }
}

impl Drop for IrqGuard {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { _mmio_critical_section_release(self.state) };
    }
}

#[inline(always)]
pub fn critical_section<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let _guard = IrqGuard::acquire();
    f()
}

/// Implement this macro once in your project to provide the critical section.
///
/// # Example (ARM Cortex-M)
///
/// ```ignore
/// mmio_rs::impl_critical_section!(
///     acquire: {
///         let primask: u32;
///         core::arch::asm!("MRS {}, PRIMASK", out(reg) primask);
///         core::arch::asm!("CPSID i", options(nomem, nostack));
///         primask
///     },
///     release: |state| {
///         core::arch::asm!("MSR PRIMASK, {}", in(reg) state, options(nomem, nostack));
///     }
/// );
/// ```
///
/// # Example (RISC-V)
///
/// ```ignore
/// mmio_rs::impl_critical_section!(
///     acquire: {
///         let mstatus: u32;
///         core::arch::asm!("csrrci {}, mstatus, 0x8", out(reg) mstatus);
///         mstatus & 0x8
///     },
///     release: |state| {
///         if state != 0 {
///             core::arch::asm!("csrsi mstatus, 0x8", options(nomem, nostack));
///         }
///     }
/// );
/// ```
///
/// # Example (host/test — no-op)
///
/// ```ignore
/// mmio_rs::impl_critical_section!(
///     acquire: { 0 },
///     release: |_state| {}
/// );
/// ```
#[macro_export]
macro_rules! impl_critical_section {
    (
        acquire: $acquire:block,
        release: |$state:ident| $release:block
    ) => {
        #[no_mangle]
        fn _mmio_critical_section_acquire() -> u32 {
            unsafe { $acquire }
        }

        #[no_mangle]
        fn _mmio_critical_section_release($state: u32) {
            unsafe { $release }
        }
    };
}
