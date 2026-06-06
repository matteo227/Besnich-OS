pub fn validate_runtime_pointers(st: &SystemTable<Boot>) -> bool {
    let ptr = unsafe { st.runtime_services() } as *const _ as u64;
    ptr >= 0x1000 && ptr <= 0xFFFF_FFFF_FFFF
}
