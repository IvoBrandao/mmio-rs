use crate::access::{Readable, Writeable};
use crate::irq_guard::critical_section;
use crate::register::Register;

pub struct AtomicRegister<'a, T: Copy, Access> {
    reg: &'a Register<T, Access>,
}

impl<'a, T, Access> AtomicRegister<'a, T, Access>
where
    T: Copy
        + core::ops::BitOr<Output = T>
        + core::ops::BitAnd<Output = T>
        + core::ops::BitXor<Output = T>
        + core::ops::Not<Output = T>,
    Access: Readable + Writeable,
{
    pub fn new(reg: &'a Register<T, Access>) -> Self {
        Self { reg }
    }

    #[inline(always)]
    pub fn set_bits(&self, mask: T) {
        critical_section(|| {
            let val = self.reg.read();
            self.reg.write(val | mask);
        });
    }

    #[inline(always)]
    pub fn clear_bits(&self, mask: T) {
        critical_section(|| {
            let val = self.reg.read();
            self.reg.write(val & !mask);
        });
    }

    #[inline(always)]
    pub fn toggle_bits(&self, mask: T) {
        critical_section(|| {
            let val = self.reg.read();
            self.reg.write(val ^ mask);
        });
    }

    #[inline(always)]
    pub fn modify<F: FnOnce(T) -> T>(&self, f: F) {
        critical_section(|| {
            let val = self.reg.read();
            self.reg.write(f(val));
        });
    }
}
