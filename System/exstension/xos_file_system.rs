use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Language {
    C,
    Cpp,
    CSharp,
    Rust,
    Python,
    Java,
    JavaScript,
    TypeScript,
    Go,
    PHP,
    Ruby,
    Kotlin,

    HTML,
    CSS,
    JSON,
    XML,

    Batch,
    PowerShell,
    VBScript,

    Text,
    Music,
    Video,

    Unknown,
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub filename: String,
    pub extension: String,
    pub language: Language,
}

pub struct XosFileSystem {
    map: HashMap<String, Language>,
}

impl XosFileSystem {
    pub fn new() -> Self {
        let mut map = HashMap::new();

        map.insert("c0x".into(), Language::C);
        map.insert("cpx".into(), Language::Cpp);
        map.insert("csxq".into(), Language::CSharp);
        map.insert("rfx".into(), Language::Rust);
        map.insert("pynx".into(), Language::Python);
        map.insert("jvx".into(), Language::Java);
        map.insert("jsxq".into(), Language::JavaScript);
        map.insert("tsxq".into(), Language::TypeScript);
        map.insert("goxr".into(), Language::Go);
        map.insert("phqx".into(), Language::PHP);
        map.insert("rbyx".into(), Language::Ruby);
        map.insert("ktlx".into(), Language::Kotlin);

        map.insert("html".into(), Language::HTML);
        map.insert("css".into(), Language::CSS);
        map.insert("json".into(), Language::JSON);
        map.insert("xml".into(), Language::XML);

        map.insert("btx".into(), Language::Batch);
        map.insert("psqx".into(), Language::PowerShell);
        map.insert("vbx".into(), Language::VBScript);

        map.insert("txo".into(), Language::Text);
        map.insert("mxa".into(), Language::Music);
        map.insert("vxo".into(), Language::Video);

        Self { map }
    }

    fn get_extension(filename: &str) -> &str {
        filename.split('.').last().unwrap_or("")
    }

    pub fn detect_language(&self, filename: &str) -> Language {
        let ext = Self::get_extension(filename);

        self.map
            .get(ext)
            .cloned()
            .unwrap_or(Language::Unknown)
    }

    pub fn analyze_file(&self, filename: &str) -> FileInfo {
        let ext = Self::get_extension(filename).to_string();
        let lang = self.detect_language(filename);

        FileInfo {
            filename: filename.to_string(),
            extension: ext,
            language: lang,
        }
    }

    pub fn print_file(&self, filename: &str) {
        let info = self.analyze_file(filename);

        println!(
            "[XOS] File: {} | Ext: .{} | Type: {:?}",
            info.filename, info.extension, info.language
        );
    }
}
