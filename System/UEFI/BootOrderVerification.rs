pub fn verify_boot_order(rt: &uefi::table::Runtime, trusted: &[u16]) -> bool {
    let mut buf = [0u8; 2];
    if rt.get_variable(cstr16!("BootCurrent"), &EFI_GLOBAL_VARIABLE, &mut buf).is_err() {
        return false;
    }
    trusted.contains(&u16::from_le_bytes(buf))
}
