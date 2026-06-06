pub fn enroll_keys(rt: &uefi::table::Runtime, db_auth: &[u8], kek_auth: &[u8], pk_auth: &[u8]) -> Result<(), Status> {
    let attrs = VariableAttributes::NON_VOLATILE
        | VariableAttributes::BOOTSERVICE_ACCESS
        | VariableAttributes::RUNTIME_ACCESS
        | VariableAttributes::TIME_BASED_AUTHENTICATED_WRITE_ACCESS;
    rt.set_variable(cstr16!("db"),  &EFI_GLOBAL_VARIABLE, attrs, db_auth).map_err(|e| e.status())?;
    rt.set_variable(cstr16!("KEK"), &EFI_GLOBAL_VARIABLE, attrs, kek_auth).map_err(|e| e.status())?;
    rt.set_variable(cstr16!("PK"),  &EFI_GLOBAL_VARIABLE, attrs, pk_auth).map_err(|e| e.status())?;
    Ok(())
}
