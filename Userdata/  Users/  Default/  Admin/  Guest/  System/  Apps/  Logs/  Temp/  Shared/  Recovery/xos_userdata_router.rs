use std::{
    fs,
    io::Write,
    path::Path,
};

fn resolve_folder(ext: &str) -> &'static str {
    match ext {
        "c0x" | "cpx" | "csxq" | "rfx" | "pynx" | "jvx" | "jsxq" | "tsxq" | "goxr" | "phqx" | "rbyx" | "ktlx" => {
            "Userdata/Apps/Code/"
        }

        "html" | "css" | "json" | "xml" => {
            "Userdata/Apps/Web/"
        }

        "btx" | "psqx" | "vbx" => {
            "Userdata/System/Scripts/"
        }

        "txo" => {
            "Userdata/Documents/Text/"
        }

        "mxa" => {
            "Userdata/Media/Music/"
        }

        "vxo" => {
            "Userdata/Media/Video/"
        }

        _ => {
            "Userdata/Unknown/"
        }
    }
}

fn ensure_structure() {
    let folders = [
        "Userdata/Apps/Code/",
        "Userdata/Apps/Web/",
        "Userdata/System/Scripts/",
        "Userdata/Documents/Text/",
        "Userdata/Media/Music/",
        "Userdata/Media/Video/",
        "Userdata/Unknown/",
    ];

    for f in folders {
        let _ = fs::create_dir_all(f);
    }
}

fn save_file(filename: &str, content: &str) {
    let ext = Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let folder = resolve_folder(ext);

    let _ = fs::create_dir_all(folder);

    let full_path = format!("{}{}", folder, filename);

    let mut file = fs::File::create(full_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn main() {
    ensure_structure();

    save_file("test.rfx", "print('Hello Rust XOS')");
    save_file("site.html", "<h1>Hello XOS</h1>");
    save_file("script.btx", "echo Hello");
    save_file("music.mxa", "AUDIO_DATA");
}
