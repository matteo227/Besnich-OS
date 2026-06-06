pub fn uninstall_unused_protocols(bt: &BootServices) {
    let guids: &[uefi::Guid] = &[
        uefi::Guid::from_values(0x6302d008,0x7f9b,0x4f30,0x87,0xac,[0x60,0xc9,0xfe,0xf5,0xda,0x4e]),
        uefi::Guid::from_values(0x752f3136,0x4e16,0x4fdc,0xa2,0x2a,[0xe5,0xf4,0x68,0x12,0xf4,0xca]),
    ];
    for guid in guids {
        if let Ok(handles) = bt.locate_handle_buffer(uefi::table::boot::SearchType::ByProtocol(guid)) {
            for handle in handles.iter() {
                let _ = bt.uninstall_protocol_interface(*handle, guid, core::ptr::null_mut());
            }
        }
    }
}
