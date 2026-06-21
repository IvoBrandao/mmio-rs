use mmio_rs::prelude::*;

mmio_rs::impl_critical_section!(
    acquire: { 0 },
    release: |_state| {}
);

fn main() {
    let mut storage: u32 = 0;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);

    reg.write(0x00FF);
    println!("Initial: 0x{:08X}", reg.read());

    // Atomic set bits (interrupt-safe OR)
    let atomic = AtomicRegister::new(&reg);
    atomic.set_bits(0xFF00);
    println!("After atomic set_bits(0xFF00): 0x{:08X}", reg.read());

    // Atomic clear bits (interrupt-safe AND with complement)
    atomic.clear_bits(0x0F0F);
    println!("After atomic clear_bits(0x0F0F): 0x{:08X}", reg.read());

    // Atomic toggle (interrupt-safe XOR)
    atomic.toggle_bits(0xFFFF);
    println!("After atomic toggle_bits(0xFFFF): 0x{:08X}", reg.read());

    // Atomic modify with custom logic
    atomic.modify(|v| (v & 0xFF00) | 0x0042);
    println!("After atomic modify: 0x{:08X}", reg.read());
}
