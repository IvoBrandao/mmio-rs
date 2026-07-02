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
fn set_bits_operation() {
    let mut storage: u32 = 0xAAAA0000;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    reg.set_bits(0x00005555);
    assert_eq!(reg.read(), 0xAAAA5555);
}

#[test]
fn and_bits_operation() {
    let mut storage: u32 = 0xFFFFFFFF;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    reg.and_bits(0x00FF00FF);
    assert_eq!(reg.read(), 0x00FF00FF);
}

#[test]
fn toggle_bits_operation() {
    let mut storage: u32 = 0xAAAAAAAA;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    reg.toggle_bits(0xFFFFFFFF);
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
fn write_field_at_shift_zero() {
    let mut storage: u32 = 0xFFFFFFFF;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    reg.write_field(0xFF, 0, 0xAB);
    assert_eq!(storage & 0xFF, 0xAB);
    assert_eq!(storage & 0xFFFFFF00, 0xFFFFFF00);
}

#[test]
fn write_field_value_clamped_to_mask() {
    // bits above the mask must not bleed into adjacent fields
    let mut storage: u32 = 0xFFFFFFFF;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    reg.write_field(0xF, 4, 0xFF); // only 4-bit mask
    assert_eq!((storage >> 4) & 0xF, 0xF);  // masked to 0xF
    assert_eq!(storage & 0xF, 0xF);          // lower nibble untouched
    assert_eq!((storage >> 8) & 0xFF, 0xFF); // upper bytes untouched
}

#[test]
fn write_field_high_bits() {
    let mut storage: u32 = 0x0000_0000;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    reg.write_field(0xF, 28, 0xA);
    assert_eq!(storage, 0xA000_0000);
}

#[test]
fn read_field() {
    let mut storage: u32 = 0x00ABCD00;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    assert_eq!(reg.read_field(0xFF, 8), 0xCD);
}

#[test]
fn read_field_at_shift_zero() {
    let mut storage: u32 = 0xABCDEF12;
    let reg: Register<u32, ReadWrite> = Register::new(&mut storage as *mut u32 as usize);
    assert_eq!(reg.read_field(0xFF, 0), 0x12);
    assert_eq!(reg.read_field(0xFFFFFFFF, 0), 0xABCDEF12);
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
fn address_round_trips() {
    let mut storage: u32 = 0;
    let addr = &mut storage as *mut u32 as usize;
    let reg: Register<u32, ReadWrite> = Register::new(addr);
    assert_eq!(reg.address(), addr);
}

#[test]
fn uninit_address_is_zero() {
    let reg: Register<u32, ReadWrite> = Register::uninit();
    assert_eq!(reg.address(), 0);
}

#[test]
fn set_address_on_uninit() {
    let mut storage: u32 = 42;
    let addr = &mut storage as *mut u32 as usize;
    let mut reg: Register<u32, ReadWrite> = Register::uninit();
    assert!(!reg.is_valid());
    reg.set_address(addr);
    assert!(reg.is_valid());
    assert_eq!(reg.address(), addr);
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
fn register_64bit() {
    let mut storage: u64 = 0;
    let reg: Register<u64, ReadWrite> = Register::new(&mut storage as *mut u64 as usize);
    reg.write(0xDEAD_BEEF_CAFE_BABE);
    assert_eq!(reg.read(), 0xDEAD_BEEF_CAFE_BABE);
    reg.set_bit(63);
    assert!(reg.get_bit(63));
    assert_eq!(reg.read_field(0xFFFF_FFFF, 32), 0xDEAD_BEEF);
}

#[test]
fn bit_ops_on_u8() {
    let mut storage: u8 = 0;
    let reg: Register<u8, ReadWrite> = Register::new(&mut storage as *mut u8 as usize);
    reg.set_bits(0xF0);
    assert_eq!(reg.read(), 0xF0);
    reg.and_bits(0xAA);
    assert_eq!(reg.read(), 0xA0);
    reg.toggle_bits(0xFF);
    assert_eq!(reg.read(), 0x5F);
}

#[test]
fn bit_ops_on_u16() {
    let mut storage: u16 = 0;
    let reg: Register<u16, ReadWrite> = Register::new(&mut storage as *mut u16 as usize);
    reg.set_bits(0xFF00);
    assert_eq!(reg.read(), 0xFF00);
    reg.and_bits(0xF0F0);
    assert_eq!(reg.read(), 0xF000);
    reg.toggle_bits(0xFFFF);
    assert_eq!(reg.read(), 0x0FFF);
    reg.write_field(0xFF, 0, 0xAB);
    assert_eq!(reg.read_field(0xFF, 0), 0xAB);
}

#[test]
fn read_only_register_reads() {
    let mut storage: u32 = 0x12345678;
    let reg: Register<u32, ReadOnly> = Register::new(&mut storage as *mut u32 as usize);
    assert_eq!(reg.read(), 0x12345678);
}

#[test]
fn read_only_get_bit_and_read_field() {
    let mut storage: u32 = 0b1010;
    let reg: Register<u32, ReadOnly> = Register::new(&mut storage as *mut u32 as usize);
    assert!(!reg.get_bit(0));
    assert!(reg.get_bit(1));
    assert_eq!(reg.read_field(0b11, 1), 0b01);

    let mut storage2: u32 = 0x00AB_CD00;
    let reg2: Register<u32, ReadOnly> = Register::new(&mut storage2 as *mut u32 as usize);
    assert_eq!(reg2.read_field(0xFF, 8), 0xCD);
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

    // all four registers
    block.reg::<u32>(2).write(0xCCCC);
    block.reg::<u32>(3).write(0xDDDD);
    assert_eq!(storage, [0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD]);
}

#[test]
fn register_block_base_and_set_base() {
    let mut storage: [u32; 2] = [0; 2];
    let base = storage.as_mut_ptr() as usize;
    let mut block = RegisterBlock::<2>::new(base, [0x00, 0x04]);
    assert_eq!(block.base(), base);

    let mut storage2: [u32; 2] = [0; 2];
    let base2 = storage2.as_mut_ptr() as usize;
    block.set_base(base2);
    assert_eq!(block.base(), base2);
    block.reg::<u32>(0).write(0x1234);
    assert_eq!(storage2[0], 0x1234);
    assert_eq!(storage[0], 0); // original storage untouched
}

#[test]
fn register_block_is_valid_when_base_zero() {
    let block = RegisterBlock::<2>::new(0, [0x00, 0x04]);
    assert!(!block.is_valid());
}

#[test]
fn register_block_reg_checked() {
    let mut storage: [u32; 2] = [0xAA, 0xBB];
    let block = RegisterBlock::<2>::new(storage.as_mut_ptr() as usize, [0x00, 0x04]);
    assert!(block.reg_checked::<u32>(0).is_some());
    assert!(block.reg_checked::<u32>(1).is_some());
    assert!(block.reg_checked::<u32>(2).is_none());
    assert_eq!(block.reg_checked::<u32>(0).unwrap().read(), 0xAA);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn register_block_reg_out_of_bounds_panics() {
    let mut storage: [u32; 2] = [0; 2];
    let block = RegisterBlock::<2>::new(storage.as_mut_ptr() as usize, [0x00, 0x04]);
    let _ = block.reg::<u32>(2); // index 2 is out of bounds for N=2
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

#[test]
fn dynamic_block_base_address() {
    let mut storage: [u32; 2] = [0; 2];
    let base = storage.as_mut_ptr() as usize;
    let block = DynamicRegisterBlock::<u32, ReadWrite, 2>::new(base, [0x00, 0x04]);
    assert_eq!(block.base_address(), base);
}

#[test]
fn dynamic_block_get_all_indices() {
    let mut storage: [u32; 3] = [0; 3];
    let block = DynamicRegisterBlock::<u32, ReadWrite, 3>::new(
        storage.as_mut_ptr() as usize,
        [0x00, 0x04, 0x08],
    );
    block.get(0).write(0x11);
    block.get(1).write(0x22);
    block.get(2).write(0x33);
    assert_eq!(storage, [0x11, 0x22, 0x33]);
}

#[test]
fn dynamic_block_get_checked() {
    let mut storage: [u32; 2] = [0xAA, 0xBB];
    let block = DynamicRegisterBlock::<u32, ReadWrite, 2>::new(
        storage.as_mut_ptr() as usize,
        [0x00, 0x04],
    );
    assert!(block.get_checked(0).is_some());
    assert!(block.get_checked(1).is_some());
    assert!(block.get_checked(2).is_none());
    assert_eq!(block.get_checked(1).unwrap().read(), 0xBB);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn dynamic_block_get_out_of_bounds_panics() {
    let mut storage: [u32; 2] = [0; 2];
    let block = DynamicRegisterBlock::<u32, ReadWrite, 2>::new(
        storage.as_mut_ptr() as usize,
        [0x00, 0x04],
    );
    let _ = block.get(2);
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

// Extended syntax: explicit type + access per field
mmio_rs::register_block! {
    struct TestStaticBlockExtended @ 0 {
        rw_reg:  u32 => ReadWrite @ 0x00,
        ro_reg:  u32 => ReadOnly  @ 0x04,
        wo_reg:  u32 => WriteOnly @ 0x08,
        narrow:  u16 => ReadWrite @ 0x0C,
    }
}

#[test]
fn static_register_block_extended_macro() {
    // Verify the accessor return types are correct (compilation proves access enforcement).
    let _: Register<u32, ReadWrite> = TestStaticBlockExtended::rw_reg();
    let _: Register<u32, ReadOnly>  = TestStaticBlockExtended::ro_reg();
    let _: Register<u32, WriteOnly> = TestStaticBlockExtended::wo_reg();
    let _: Register<u16, ReadWrite> = TestStaticBlockExtended::narrow();
    assert_eq!(TestStaticBlockExtended::ro_reg().address(), 0x04);
    assert_eq!(TestStaticBlockExtended::narrow().address(), 0x0C);
}

mmio_rs::register_block! {
    struct TestDynamicBlock {
        csr: 0x00,
        rvr: 0x04,
        cvr: 0x08,
    }
}

// Extended syntax: explicit type + access per field
mmio_rs::register_block! {
    struct TestDynamicBlockExtended {
        ctrl: u32 => ReadWrite @ 0x00,
        stat: u32 => ReadOnly  @ 0x04,
        data: u16 => ReadWrite @ 0x08,
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

#[test]
fn dynamic_register_block_extended_macro() {
    // Use a [u32; 3] but access `data` as u16 at offset 0x08 (storage[2] low half).
    let mut storage: [u32; 3] = [0; 3];
    let base = storage.as_mut_ptr() as usize;
    let block = TestDynamicBlockExtended::new(base);
    assert!(block.is_valid());

    let _: Register<u32, ReadWrite> = block.ctrl();
    let _: Register<u32, ReadOnly>  = block.stat();
    let _: Register<u16, ReadWrite> = block.data();

    block.ctrl().write(0xDEAD_BEEF);
    assert_eq!(storage[0], 0xDEAD_BEEF);
    // stat is ReadOnly so we verify address is correct, not write
    assert_eq!(block.stat().address(), base + 0x04);
}

#[test]
fn dynamic_register_block_extended_macro_late_init() {
    let mut block = TestDynamicBlockExtended::uninit();
    assert!(!block.is_valid());
    let mut storage: [u32; 3] = [0; 3];
    block.set_base_address(storage.as_mut_ptr() as usize);
    assert!(block.is_valid());
    block.ctrl().write(0x1234_5678);
    assert_eq!(storage[0], 0x1234_5678);
}

#[test]
fn irq_guard_acquires_and_releases() {
    // Verify the guard can be acquired and released without panicking.
    // In this test environment the critical section is a no-op.
    {
        let _guard = IrqGuard::acquire();
        // guard released here via Drop
    }
    // A second acquisition must work (would deadlock/panic if not released).
    let _guard2 = IrqGuard::acquire();
}

#[test]
fn critical_section_returns_value() {
    let result: u32 = mmio_rs::critical_section(|| 0xCAFE_BABE);
    assert_eq!(result, 0xCAFE_BABE);
}
