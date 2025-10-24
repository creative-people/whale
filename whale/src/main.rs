mod whale_app;
mod chess_engine;
mod chess_parts;

use whale_app::WhaleApp;
use eframe;

fn main() {
    let _ = eframe::run_native(
        "Whale Chess",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<WhaleApp>::new(WhaleApp::new()))),
    );
}
