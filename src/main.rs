use macroquad::prelude::*;
use window_manager::WindowManager;
use window_style::WindowStyle;
mod widget;
mod window;
mod window_manager;
mod window_style;

#[macroquad::main("Another UI Library")]
async fn main() {
    let mut windows = WindowManager::new();
	let style = WindowStyle {
		titlebar: Color::from_hex(0x781D1D),
		resize_triangle: Color::from_hex(0x5C0E0E),
		outline: DARKGRAY,
		selected_outline: Color::from_hex(0xff4444),
		button_bg_color: Color::from_hex(0x781D1D),
		..Default::default()
	};

    loop {
        windows.begin("Hello Window", |win| {
			if win.once {
				win.set_style(&style);
				win.set_size((250., 106.));
			}

            let row = win.row();
            row.text("Mouse Position");
            row.text("|").color = GRAY;
            row.text("X:");
            row.text(mouse_position().0).color = RED;
            row.text(", Y:");
            row.text(mouse_position().1).color = BLUE;

			win.text("5 Big Booms");
			win.button("BOOM!");
        });
        windows.end_windows();

        next_frame().await
    }
}
