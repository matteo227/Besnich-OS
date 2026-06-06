pub fn set_watchdog(bt: &BootServices, seconds: usize) {
    let _ = bt.set_watchdog_timer(seconds, 0x10000, None);
}

pub fn disable_watchdog(bt: &BootServices) {
    let _ = bt.set_watchdog_timer(0, 0x10000, None);
}
