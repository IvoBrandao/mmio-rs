mmio_rs::impl_critical_section!(
    acquire: { 0 },
    release: |_state| {}
);

mmio_rs::register_block! {
    /// SPI peripheral register layout
    struct SpiRegs {
        cr1: 0x00,
        cr2: 0x04,
        sr:  0x08,
        dr:  0x0C,
    }
}

struct SpiDriver {
    regs: SpiRegs,
}

impl SpiDriver {
    fn new(base: usize) -> Self {
        Self {
            regs: SpiRegs::new(base),
        }
    }

    fn init(&self, prescaler: u32) {
        self.regs.cr1().write(prescaler << 3);
        self.regs.cr1().set_bit(6); // SPE — enable SPI
    }

    fn transfer(&self, tx: u8) -> u8 {
        // In real hardware: wait for TXE, write DR, wait for RXNE, read DR
        self.regs.dr().write(tx as u32);
        self.regs.dr().read() as u8
    }

    fn disable(&self) {
        self.regs.cr1().clear_bit(6);
    }
}

fn main() {
    let mut hw: [u32; 4] = [0; 4];
    let spi = SpiDriver::new(hw.as_mut_ptr() as usize);

    spi.init(0b011); // prescaler /16
    println!("CR1 after init: 0x{:08X}", hw[0]);

    let rx = spi.transfer(0x55);
    println!("Transferred 0x55, got back: 0x{:02X}", rx);

    spi.disable();
    println!("CR1 after disable: 0x{:08X}", hw[0]);
}
