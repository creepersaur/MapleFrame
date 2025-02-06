use macroquad::prelude::*;
use mapleframe::prelude::*;

#[macroquad::main("MapleFrame")]
async fn main() {
    let mut windows = WindowManager::new();
    let mut toggle = false;

    loop {
        windows.begin("explorer", |win| {
            win.set_position((0., 0.));
            win.set_size((250., 300.));
            bald(win);

            win.text("Hello");
            win.separator();
            win.text("Hello");
            win.separator();

            win.row(|row| {
				row.text("Toggle on:");
                if row.button(toggle).clicked {
                    toggle = !toggle;
                }
            });

			win.indent(20., |indent| {
				indent.text("Hello world");
				indent.indent(20.);
				indent.text("Hello world");
			});
        });

        windows.begin("properties", |win| {
            win.set_position((0., 300.));
            win.set_size((250., 300.));
            bald(win);

            win.row(|row| {
                row.text("Order:");
                row.button("Burger");
                row.separator();
                row.button("Nuggets");
            });

            win.separator();

            win.row(|row| {
            row.text("Order:");
            row.button("Burger");
            row.separator();
            row.button("Nuggets");
		});
        });

        windows.begin("output", |win| {
            win.set_position((250., 600. - 150.));
            win.set_size((800. - 250., 150.));
            bald(win);
        });

        windows.end_windows();
        next_frame().await
    }
}

fn bald(win: &mut Window) {
    // win.resizable = false;
    win.closable = false;
    win.draggable = false;
    win.minimizable = false;
}
