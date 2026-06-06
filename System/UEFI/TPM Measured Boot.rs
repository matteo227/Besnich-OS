pub fn tpm_extend_pcr(bt: &BootServices, pcr_index: u32, data: &[u8]) -> Result<(), Status> {
    let handle = bt.get_handle_for_protocol::<RawTcg2>().map_err(|e| e.status())?;
    let mut tcg2 = bt.open_protocol_exclusive::<RawTcg2>(handle).map_err(|e| e.status())?;
    let event = Tcg2Event {
        size: core::mem::size_of::<Tcg2Event>() as u32,
        header: Tcg2EventHeader { header_size: core::mem::size_of::<Tcg2EventHeader>() as u32, header_version: 1, pcr_index, event_type: 0xD },
        event: [0u8; 64],
    };
    unsafe { ((*tcg2.as_mut_ptr()).hash_log_extend_event)(tcg2.as_mut_ptr(), 0, data.as_ptr() as u64, data.len() as u64, &event) }
        .to_result().map_err(|e| e.status())
}
