pub fn block_unsigned_capsules(rt: &uefi::table::Runtime) -> bool {
    let mut buf = [0u8; 4];
    match rt.get_variable(cstr16!("OsIndicationsSupported"), &EFI_GLOBAL_VARIABLE, &mut buf) {
        Ok(_) => u32::from_le_bytes(buf) & 0x4 == 0,
        Err(_) => true,
    }
}
