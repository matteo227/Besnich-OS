pub fn verify_boot_services_integrity(st: &SystemTable<Boot>) -> bool {
    let ptr = st.boot_services() as *const BootServices as u64;
    ptr >= 0x100000
}
