use std::fs::read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Searches for fonts.
pub struct FontSearcher {
    pub fonts: Vec<(PathBuf, Vec<u8>)>,
}

impl FontSearcher {
    /// Create a new, empty system searcher.
    pub fn new() -> Self {
        Self { fonts: vec![] }
    }

    #[cfg(not(any(
        target_os = "macos",
        target_os = "windows",
        target_os = "linux",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    )))]
    pub fn search_system(&mut self) {}

    /// Search for fonts in netbsd/openbsd etc.
    #[cfg(any(target_os = "netbsd", target_os = "openbsd"))]
    pub fn search_system(&mut self) {
        self.search_dir("/usr/X11R7/lib/X11/fonts");
        self.search_dir("/usr/X11R7/lib/X11/fonts/OTF");
        self.search_dir("/usr/X11R7/lib/X11/fonts/TTF");
        self.search_dir("/usr/X11R7/lib/X11/fonts/Type1");

        if let Some(dir) = dirs::font_dir() {
            self.search_dir(dir);
        }
    }

    /// Search for fonts in fedora/ubuntu/freebsd etc.
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn search_system(&mut self) {
        self.search_dir("/usr/share/fonts");

        if let Some(dir) = dirs::font_dir() {
            self.search_dir(dir);
        }
    }

    /// Search for fonts in the macOS system font directories.
    #[cfg(target_os = "macos")]
    pub fn search_system(&mut self) {
        self.search_dir("/Library/Fonts");
        self.search_dir("/Network/Library/Fonts");
        self.search_dir("/System/Library/Fonts");

        if let Some(dir) = dirs::font_dir() {
            self.search_dir(dir);
        }
    }

    /// Search for fonts in the Windows system font directories.
    #[cfg(target_os = "windows")]
    pub fn search_system(&mut self) {
        let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());

        self.search_dir(Path::new(&windir).join("Fonts"));

        if let Some(roaming) = dirs::config_dir() {
            self.search_dir(roaming.join("Microsoft\\Windows\\Fonts"));
        }

        if let Some(local) = dirs::cache_dir() {
            self.search_dir(local.join("Microsoft\\Windows\\Fonts"));
        }
    }

    /// Search for all fonts in a directory recursively.
    pub fn search_dir(&mut self, path: impl AsRef<Path>) {
        for entry in WalkDir::new(path)
            .follow_links(true)
            .sort_by(|a, b| a.file_name().cmp(b.file_name()))
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if matches!(
                path.extension().and_then(|s| s.to_str()),
                Some("ttf" | "otf" | "TTF" | "OTF" | "ttc" | "otc" | "TTC" | "OTC"),
            ) {
                self.search_file(path);
            }
        }
    }

    #[cfg(any(target_os = "windows", target_os = "linux"))]
    fn contains_cjk(s: &str) -> bool {
        for c in s.chars() {
            if c >= '\u{4e00}' && c <= '\u{9fff}' {
                return true;
            }
        }
        false
    }

    #[cfg(any(target_os = "windows", target_os = "linux"))]
    /// Index the fonts in the file at the given path.
    pub fn search_file(&mut self, path: impl AsRef<Path>) {
        let content = read(path.as_ref()).unwrap();
        let count = ttf_parser::fonts_in_collection(&content).unwrap_or(1);
        for index in 0..count {
            if let Ok(x) = ttf_parser::Face::parse(&content, index) {
                for x in x.names() {
                    if x.name_id != 4 {
                        continue;
                    }
                    if x.to_string().map(|x| Self::contains_cjk(&x)) == Some(true) {
                        self.fonts.push((path.as_ref().to_path_buf(), content));
                        return;
                    }
                }
            }
        }
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    /// Index the fonts in the file at the given path.
    pub fn search_file(&mut self, path: impl AsRef<Path>) {
        let content = read(path.as_ref()).unwrap();
        self.fonts.push((path.as_ref().to_path_buf(), content));
    }
}
