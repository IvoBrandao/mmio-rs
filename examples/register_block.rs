mmio_rs::impl_critical_section!(
    acquire: { 0 },
    release: |_state| {}
);

// Mode 1: Static block — base address known at compile time
mmio_rs::register_block! {
    /// SysTick timer registers (Cortex-M fixed address)
    pub struct SysTick @ 0xE000_E010 {
        csr: 0x00,
        rvr: 0x04,
        cvr: 0x08,
        calib: 0x0C,
    }
}

// Mode 2 & 3: Dynamic block — base address set at runtime
mmio_rs::register_block! {
    /// UART peripheral registers (base varies per instance)
    pub struct UartRegs {
        sr:  0x00,
        dr:  0x04,
        brr: 0x08,
        cr1: 0x0C,
        cr2: 0x10,
    }
}

fn main() {
    // --- Mode 1: Static base (compile-time) ---
    println!("=== Static Block (SysTick) ===");
    println!("CSR address: 0x{:08X}", SysTick::csr().address());
    println!("RVR address: 0x{:08X}", SysTick::rvr().address());

    // --- Mode 2: Constructor (runtime base) ---
    println!("\n=== Dynamic Block (UART1) ===");
    let mut uart1_hw: [u32; 5] = [0; 5];
    let uart1 = UartRegs::new(uart1_hw.as_mut_ptr() as usize);
    assert!(uart1.is_valid());

    uart1.cr1().write(0x0000_200C); // enable UART, TX, RX
    uart1.brr().write(0x0683);       // 9600 baud @ 16MHz
    uart1.dr().write(b'A' as u32);

    println!("CR1: 0x{:08X}", uart1_hw[3]);
    println!("BRR: 0x{:08X}", uart1_hw[2]);
    println!("DR:  0x{:08X}", uart1_hw[1]);

    // --- Mode 3: Late initialization (two-phase) ---
    println!("\n=== Late Init (UART2) ===");
    let mut uart2 = UartRegs::uninit();
    assert!(!uart2.is_valid());

    let mut uart2_hw: [u32; 5] = [0; 5];
    uart2.set_base_address(uart2_hw.as_mut_ptr() as usize);
    assert!(uart2.is_valid());

    uart2.cr1().write(0x0000_200C);
    println!("UART2 CR1: 0x{:08X}", uart2_hw[3]);
}
