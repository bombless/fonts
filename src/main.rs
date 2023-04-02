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
    gpu_cache::main(font_searcher.fonts).unwrap();
}
