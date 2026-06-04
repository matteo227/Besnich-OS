use std::process::Command;

fn get_gpu_info() -> String {
    let output = Command::new("nvidia-smi")
        .arg("--query-gpu=name,memory.used,memory.total,utilization.gpu")
        .arg("--format=csv,noheader,nounits")
        .output();

    match output {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(_) => String::from("GPU info non disponibile"),
    }
}

fn main() {
    println!("GPU:");
    println!("{}", get_gpu_info());
}
