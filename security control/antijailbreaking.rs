use sha2::{Sha256, Digest};
use std::{
    fs,
    path::Path,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

const ANTICHEAT_PATH: &str = "C:\\BESNICH\\antijailbreaking\\";
const SYSTEM_IMAGE: &str = "C:\\BESNICH\\recovery\\clean.img";
const USERDATA_PATH: &str = "C:\\Users\\";

// hash attesi generati dall'installer ufficiale
const EXPECTED_HASH_DB: &[(&str, &str)] = &[
    ("kernel.bin", "a1b2c3d4..."),
    ("policy.bin", "e5f6g7h8..."),
];

fn hash_file(path: &str) -> Option<String> {
    let data = fs::read(path).ok()?;
    let mut hasher = Sha256::new();
    hasher.update(data);
    Some(format!("{:x}", hasher.finalize()))
}

fn verify_antijailbreaking_integrity() -> bool {
    for (file, expected) in EXPECTED_HASH_DB {
        let full_path = format!("{}{}", ANTICHEAT_PATH, file);

        let hash = match hash_file(&full_path) {
            Some(h) => h,
            None => return false,
        };

        if &hash != expected {
            return false;
        }
    }
    true
}

fn detect_tampering_signals() -> bool {
    // segnali reali su Windows
    let suspicious_paths = [
        "C:\\Windows\\System32\\cmd.exe",
        "C:\\Windows\\Temp\\rootkit.tmp",
        "C:\\ProgramData\\jailbreak.flag",
    ];

    for p in suspicious_paths {
        if Path::new(p).exists() {
            return true;
        }
    }

    false
}

fn backup_user_data() {
    let _ = Command::new("robocopy")
        .args(&[
            USERDATA_PATH,
            "C:\\BESNICH\\backup\\users\\",
            "/MIR",
        ])
        .spawn();
}

fn restore_clean_system() {
    println!("[BESNICH OS] Compromissione rilevata -> restore");

    backup_user_data();

    let _ = Command::new("besnich_recovery_tool.exe")
        .args(&["--image", SYSTEM_IMAGE])
        .spawn();

    let _ = Command::new("robocopy")
        .args(&[
            "C:\\BESNICH\\backup\\users\\",
            USERDATA_PATH,
            "/MIR",
        ])
        .spawn();
}

fn is_update_mode() -> bool {
    Path::new("C:\\BESNICH\\update.lock").exists()
}

fn lock_antijailbreaking() {
    let _ = Command::new("attrib")
        .args(&["+R", "+H", ANTICHEAT_PATH])
        .spawn();
}

fn unlock_antijailbreaking() {
    let _ = Command::new("attrib")
        .args(&["-R", "-H", ANTICHEAT_PATH])
        .spawn();
}

fn main() {
    // modalità update ufficiale
    if is_update_mode() {
        unlock_antijailbreaking();
        println!("[BESNICH OS] Update mode attivo");
        return;
    }

    // controllo integrità
    if !verify_antijailbreaking_integrity() {
        restore_clean_system();
        return;
    }

    // rilevamento jailbreak/tampering
    if detect_tampering_signals() {
        restore_clean_system();
        return;
    }

    lock_antijailbreaking();
    println!("[BESNICH OS] Sistema protetto e integro");
}
