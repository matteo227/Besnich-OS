use sysinfo::{System, SystemExt, CpuExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("CPU:");

    for (i, cpu) in sys.cpus().iter().enumerate() {
        println!("Core {}: {}%", i, cpu.cpu_usage());
    }
}
