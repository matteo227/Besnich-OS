pub fn check_secure_boot_state(st: &SystemTable<Boot>) -> SecureBootState {
    let rt = unsafe { st.runtime_services() };
    let mut sb_buf = [0u8; 1];
    let mut sm_buf = [0u8; 1];
    let sb = rt.get_variable(cstr16!("SecureBoot"), &EFI_GLOBAL_VARIABLE, &mut sb_buf);
    let sm = rt.get_variable(cstr16!("SetupMode"),  &EFI_GLOBAL_VARIABLE, &mut sm_buf);
    match (sb, sm) {
        (Ok(_), Ok(_)) => match (sb_buf[0], sm_buf[0]) {
            (_, 1) => SecureBootState::SetupMode,
            (1, 0) => SecureBootState::Enabled,
            (0, 0) => SecureBootState::Disabled,
            _      => SecureBootState::Unknown,
        },
        _ => SecureBootState::Unknown,
    }
}
