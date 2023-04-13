use std::path::{PathBuf, Path};
use crate::{fc, font};
use std::process::exit;
use std::fs::read;

/// Searches for fonts.
pub struct FontSearcher {
    pub fonts: Vec<(PathBuf, Vec<u8>)>,
}

impl FontSearcher {
    /// Create a new, empty system searcher.
    pub fn new() -> Self {
        Self { fonts: vec![] }
    }

    pub fn search_system(&mut self) {
        fc::init().unwrap_or_else(|_| {
            eprintln!("init FontConfig failed");
            exit(1);
        });
    
        let char = '你';
    
        let charset = fc::Charset::default().add_char(char);
        let pattern = fc::Pattern::default().add_charset(&charset);
        let font_set = fc::FontSet::match_pattern(&pattern);
    
        let families = font::SortedFamilies::from(&font_set);
    
        if families.is_empty() {
            println!("No font support this character.");
            return;
        }
    
        for family in families {
            for f in family.fonts {

                let content = read(f.0.path).unwrap();

                self.fonts.push((Path::new(f.0.path).into(), content))
            }
        }
    
        fc::finalize();
    }

}
