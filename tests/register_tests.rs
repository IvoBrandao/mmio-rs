use mmio_rs::prelude::*;

mmio_rs::impl_critical_section!(
    acquire: { 0 },
    release: |_state| {}
);

#[test]
fn write_and_read() {
    let mut storage: u32 = 0;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    reg.write(0xDEADBEEF);
    assert_eq!(reg.read(), 0xDEADBEEF);
}

#[test]
fn read_initial_value() {
    let mut storage: u32 = 0xCAFEBABE;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    assert_eq!(reg.read(), 0xCAFEBABE);
}

#[test]
fn set_bit() {
    let mut storage: u32 = 0;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    reg.set_bit(0);
    assert_eq!(reg.read(), 1);
    reg.set_bit(31);
    assert_eq!(reg.read(), 0x80000001);
}

#[test]
fn clear_bit() {
    let mut storage: u32 = 0xFF;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    reg.clear_bit(0);
    assert_eq!(reg.read(), 0xFE);
    reg.clear_bit(7);
    assert_eq!(reg.read(), 0x7E);
}

#[test]
fn get_bit() {
    let mut storage: u32 = 0b1010;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    assert!(!reg.get_bit(0));
    assert!(reg.get_bit(1));
    assert!(!reg.get_bit(2));
    assert!(reg.get_bit(3));
}

#[test]
fn or_operation() {
    let mut storage: u32 = 0xAAAA0000;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    reg.or(0x00005555);
    assert_eq!(reg.read(), 0xAAAA5555);
}

#[test]
fn and_operation() {
    let mut storage: u32 = 0xFFFFFFFF;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    reg.and(0x00FF00FF);
    assert_eq!(reg.read(), 0x00FF00FF);
}

#[test]
fn xor_operation() {
    let mut storage: u32 = 0xAAAAAAAA;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    reg.xor(0xFFFFFFFF);
    assert_eq!(reg.read(), 0x55555555);
}

#[test]
fn modify_closure() {
    let mut storage: u32 = 0x1234;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    reg.modify(|v| (v & 0xFF00) | 0x0056);
    assert_eq!(reg.read(), 0x1256);
}

#[test]
fn write_field() {
    let mut storage: u32 = 0xFFFFFFFF;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    reg.write_field(0xF, 8, 0x5);
    assert_eq!((storage >> 8) & 0xF, 0x5);
    assert_eq!(storage & 0xFF, 0xFF);
}

#[test]
fn read_field() {
    let mut storage: u32 = 0x00ABCD00;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    assert_eq!(reg.read_field(0xFF, 8), 0xCD);
}

#[test]
fn is_valid_and_uninit() {
    let reg: Register<u32, ReadWrite> = Register::uninit();
    assert!(!reg.is_valid());

    let mut storage: u32 = 0;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    assert!(reg.is_valid());
}

#[test]
fn set_address_on_uninit() {
    let mut storage: u32 = 42;
    let mut reg: Register<u32, ReadWrite> = Register::uninit();
    assert!(!reg.is_valid());
    reg.set_address(&mut storage as *mut u32 as usize);
    assert!(reg.is_valid());
    assert_eq!(reg.read(), 42);
}

#[test]
fn register_8bit() {
    let mut storage: u8 = 0;
    let reg: Register<u8, ReadWrite> = Register::new(&mut storage as *mut u8 as usize);
    reg.write(0xAB);
    assert_eq!(reg.read(), 0xAB);
    reg.set_bit(7);
    assert_eq!(reg.read(), 0xAB);
    reg.write(0);
    reg.set_bit(7);
    assert_eq!(reg.read(), 0x80);
}

#[test]
fn register_16bit() {
    let mut storage: u16 = 0;
    let reg: Register<u16, ReadWrite> = Register::new(&mut storage as *mut u16 as usize);
    reg.write(0xBEEF);
    assert_eq!(reg.read(), 0xBEEF);
}

#[test]
fn read_only_register_reads() {
    let mut storage: u32 = 0x12345678;
    let reg: Register<u32, ReadOnly> = Register::new(&mut storage as *mut u32 as usize);
    assert_eq!(reg.read(), 0x12345678);
}

#[test]
fn write_only_register_writes() {
    let mut storage: u32 = 0;
    let reg: Register<u32, WriteOnly> = Register::new(&mut storage as *mut u32 as usize);
    reg.write(0xABCD);
    assert_eq!(storage, 0xABCD);
}

#[test]
fn atomic_set_bits() {
    let mut storage: u32 = 0x00FF;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    let atomic = AtomicRegister::new(&reg);
    atomic.set_bits(0xFF00);
    assert_eq!(reg.read(), 0xFFFF);
}

#[test]
fn atomic_clear_bits() {
    let mut storage: u32 = 0xFFFF;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    let atomic = AtomicRegister::new(&reg);
    atomic.clear_bits(0x00FF);
    assert_eq!(reg.read(), 0xFF00);
}

#[test]
fn atomic_toggle_bits() {
    let mut storage: u32 = 0xAAAA;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    let atomic = AtomicRegister::new(&reg);
    atomic.toggle_bits(0xFFFF);
    assert_eq!(reg.read(), 0x5555);
}

#[test]
fn atomic_modify() {
    let mut storage: u32 = 0x1234;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    let atomic = AtomicRegister::new(&reg);
    atomic.modify(|v| (v & 0xFF00) | 0x0056);
    assert_eq!(reg.read(), 0x1256);
}

#[test]
fn register_block_generic() {
    let mut storage: [u32; 4] = [0; 4];
    let block = RegisterBlock::<4>::new(
        storage.as_mut_ptr() as usize,
        [0x00, 0x04, 0x08, 0x0C],
    );
    assert!(block.is_valid());

    let r0: Register<u32, ReadWrite> = block.reg(0);
    r0.write(0xAAAA);
    assert_eq!(storage[0], 0xAAAA);

    let r1: Register<u32, ReadWrite> = block.reg(1);
    r1.write(0xBBBB);
    assert_eq!(storage[1], 0xBBBB);
}

#[test]
fn dynamic_block_late_init() {
    let mut storage: [u32; 3] = [0; 3];
    let mut block = DynamicRegisterBlock::<u32, ReadWrite, 3>::uninit([0x00, 0x04, 0x08]);
    assert!(!block.is_valid());
    block.set_base_address(storage.as_mut_ptr() as usize);
    assert!(block.is_valid());

    let r: Register<u32, ReadWrite> = block.get(0);
    r.write(42);
    assert_eq!(storage[0], 42);
}

mmio_rs::register_block! {
    struct TestStaticBlock @ 0 {
        reg_a: 0x00,
        reg_b: 0x04,
    }
}

#[test]
fn static_register_block_macro_compiles() {
    let _a = TestStaticBlock::reg_a();
    let _b = TestStaticBlock::reg_b();
}

mmio_rs::register_block! {
    struct TestDynamicBlock {
        csr: 0x00,
        rvr: 0x04,
        cvr: 0x08,
    }
}

#[test]
fn dynamic_register_block_macro() {
    let mut storage: [u32; 3] = [0; 3];
    let block = TestDynamicBlock::new(storage.as_mut_ptr() as usize);
    assert!(block.is_valid());

    let csr = block.csr();
    csr.write(0x1234);
    assert_eq!(storage[0], 0x1234);

    let rvr = block.rvr();
    rvr.write(999);
    assert_eq!(storage[1], 999);
}

#[test]
fn dynamic_register_block_macro_late_init() {
    let mut storage: [u32; 3] = [0; 3];
    let mut block = TestDynamicBlock::uninit();
    assert!(!block.is_valid());
    block.set_base_address(storage.as_mut_ptr() as usize);
    assert!(block.is_valid());
    block.csr().write(0xABCD);
    assert_eq!(storage[0], 0xABCD);
}
