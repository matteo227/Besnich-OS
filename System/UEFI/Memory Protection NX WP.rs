pub fn apply_memory_protections(bt: &BootServices) {
    let mut map_buf = [0u8; 16384];
    if let Ok(map) = bt.memory_map(&mut map_buf) {
        for desc in map.entries() {
            match desc.ty {
                MemoryType::CONVENTIONAL => {
                    let _ = bt.set_memory_attributes(desc.phys_start, desc.page_count * 4096, MemoryAttribute::XP);
                }
                MemoryType::LOADER_CODE | MemoryType::BOOT_SERVICES_CODE => {
                    let _ = bt.set_memory_attributes(desc.phys_start, desc.page_count * 4096, MemoryAttribute::RO);
                }
                _ => {}
            }
        }
    }
}
