mod gpu_cache;
mod font_searcher;
mod fc;
mod font;

fn main() {
    let mut font_searcher = font_searcher::FontSearcher::new();
    font_searcher.search_system();
    // for f in &font_searcher.fonts {
    //     println!("{:?}", f.0)
    // }
    gpu_cache::main(font_searcher.fonts).unwrap();
}
