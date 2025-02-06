use macroquad::prelude::*;
use mapleframe::prelude::*;

#[macroquad::main("MapleFrame")]
async fn main() {
    let mut windows = WindowManager::new();

    loop {
        windows.begin("Debug", |win| {
			win.text("Hello world");
        });

        windows.end_windows();
        next_frame().await
    }
}