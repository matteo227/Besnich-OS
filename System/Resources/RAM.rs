use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_ram = sys.total_memory() / 1024;
    let used_ram = sys.used_memory() / 1024;

    println!("RAM:");
    println!("Used: {} MB", used_ram);
    println!("Total: {} MB", total_ram);
}
