use macroquad::prelude::*;
use window_manager::WindowManager;
mod widget;
mod window;
mod window_manager;
mod window_style;

pub mod prelude;

#[macroquad::main("Another UI Library")]
async fn main() {
    let mut windows = WindowManager::new();

    loop {
        windows.begin("explorer", |win| {
            win.set_position((0., 0.));
            win.set_size((250., 300.));
            win.resizable = false;
            win.closable = false;
            win.draggable = false;
            win.minimizable = false;
        });

        windows.begin("properties", |win| {
            win.set_position((0., 300.));
            win.set_size((250., 300.));
            win.resizable = false;
            win.closable = false;
            win.draggable = false;
            win.minimizable = false;
        });

		windows.begin("output", |win| {
            win.set_position((250., 600. - 150.));
            win.set_size((800. - 250., 150.));
            win.resizable = false;
            win.closable = false;
            win.draggable = false;
            win.minimizable = false;
        });

        windows.end_windows();
        next_frame().await
    }
}
