mod gpu_cache;
mod fonts;

fn main() {
    let mut font_searcher = fonts::FontSearcher::new();
    font_searcher.search_system();
    gpu_cache::main(font_searcher.fonts).unwrap();
}
