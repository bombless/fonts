mod gpu_cache;
mod font_searcher;
mod fc;
mod font;

use std::path::Path;
use std::fs::read;

fn main() {
    let mut font_searcher = font_searcher::FontSearcher::new();
    font_searcher.search_system();
    // for f in &font_searcher.fonts {
    //     println!("{:?}", f.0)
    // }
    let font = read(Path::new("../font-140507875434664")).unwrap();
    font_searcher.fonts.insert(0, ("../font-140507875434664".into(), font));
    gpu_cache::main(font_searcher.fonts).unwrap();
}
