#![no_main]
#![no_std]

use log::{error, info, warn};
use uefi::{
    prelude::*,
    table::runtime::VariableVendor,
    Status,
};

const EFI_GLOBAL_VARIABLE: VariableVendor = VariableVendor(uefi::Guid::from_values(
    0x8be4df61,
    0x93ca,
    0x11d2,
    0xaa,
    0x0d,
    [0x00, 0xe0, 0x98, 0x03, 0x2b, 0x8c],
));

#[derive(Debug)]
pub enum SecureBootState {
    Enabled,
    Disabled,
    SetupMode,
    Unknown,
}

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

pub fn enroll_keys(
    rt: &uefi::table::Runtime,
    db_auth:  &[u8],
    kek_auth: &[u8],
    pk_auth:  &[u8],
) -> Result<(), Status> {
    use uefi::table::runtime::VariableAttributes;

    let attrs = VariableAttributes::NON_VOLATILE
        | VariableAttributes::BOOTSERVICE_ACCESS
        | VariableAttributes::RUNTIME_ACCESS
        | VariableAttributes::TIME_BASED_AUTHENTICATED_WRITE_ACCESS;

    rt.set_variable(cstr16!("db"),  &EFI_GLOBAL_VARIABLE, attrs, db_auth)
        .map_err(|e| e.status())?;
    rt.set_variable(cstr16!("KEK"), &EFI_GLOBAL_VARIABLE, attrs, kek_auth)
        .map_err(|e| e.status())?;
    rt.set_variable(cstr16!("PK"),  &EFI_GLOBAL_VARIABLE, attrs, pk_auth)
        .map_err(|e| e.status())?;

    info!("[OK] Chiavi enrolled");
    Ok(())
}

#[entry]
fn main(image: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut st).unwrap();

    match check_secure_boot_state(&st) {
        SecureBootState::Enabled  => { info!("[OK] Secure Boot attivo"); }
        SecureBootState::Disabled => { warn!("[!!] Secure Boot disabilitato"); }
        SecureBootState::SetupMode => {
            info!("[i] Setup Mode — enroll chiavi...");
        }
        SecureBootState::Unknown  => {
            error!("[ERR] Stato sconosciuto");
            return Status::UNSUPPORTED;
        }
    }

    Status::SUCCESS
}
