mod gpu_cache;
mod fonts;

use std::path::Path;
use std::fs::read;

fn main() {
    let mut font_searcher = fonts::FontSearcher::new();
    font_searcher.search_system();
    // for f in &font_searcher.fonts {
    //     println!("{:?}", f.0)
    // }
    let font = read(Path::new("../font-140507875434664")).unwrap();
    font_searcher.fonts.insert(0, ("../font-140507875434664".into(), font));
    gpu_cache::main(font_searcher.fonts).unwrap();
}
