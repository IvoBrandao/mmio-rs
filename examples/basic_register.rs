use mmio_rs::prelude::*;

mmio_rs::impl_critical_section!(
    acquire: { 0 },
    release: |_state| {}
);

fn main() {
    let mut storage: u32 = 0;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);

    // Write and read
    reg.write(0x0001);
    println!("Value: 0x{:08X}", reg.read());

    // Bit manipulation
    reg.set_bit(3);
    println!("After set_bit(3): 0x{:08X}", reg.read());

    reg.clear_bit(0);
    println!("After clear_bit(0): 0x{:08X}", reg.read());

    println!("Bit 3 is: {}", reg.get_bit(3));

    // Field access (bits [7:4])
    reg.write(0);
    reg.write_field(0xF, 4, 0xA);
    println!("After write_field(0xF, 4, 0xA): 0x{:08X}", reg.read());
    println!("Read field: 0x{:X}", reg.read_field(0xF, 4));

    // Compound operations
    reg.write(0xFF00);
    reg.set_bits(0x00FF);
    println!("After set_bits(0x00FF): 0x{:08X}", reg.read());

    reg.and_bits(0x0F0F);
    println!("After and_bits(0x0F0F): 0x{:08X}", reg.read());

    reg.toggle_bits(0xFFFF);
    println!("After toggle_bits(0xFFFF): 0x{:08X}", reg.read());
}
