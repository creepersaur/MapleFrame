use macroquad::prelude::*;
use mapleframe::prelude::*;

#[macroquad::main("MapleFrame")]
async fn main() {
    let mut windows = WindowManager::new();
    let mut t1 = false;
    let mut t2 = false;

    loop {
        windows.begin("explorer", |win| {
            win.set_position((0., 0.));
            win.set_size((250., 300.));
            bald(win);

            win.row(|row| {
                if row
                    .button(match t1 {
                        true => "\\/",
                        false => ">",
                    })
                    .clicked
                {
                    t1 = !t1
                }
                row.text("workspace");
            });

            if t1 {
                win.indent(24., |indent| {
                    for i in 1..2 {
                        indent.button(format!("Object{i}"));
                    }
                });
            }

            win.separator();

            win.row(|row| {
                if row
                    .button(match t2 {
                        true => "\\/",
                        false => ">",
                    })
                    .clicked
                {
                    t2 = !t2
                }
                row.text("workspace");
            });

            if t2 {
                win.indent(24., |indent| {
                    for i in 1..2 {
                        indent.button(format!("Object{i}"));
                    }
                });
            }
        });

        windows.begin("properties", |win| {
            win.set_position((0., 300.));
            win.set_size((250., 300.));
            bald(win);

            win.separator();

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

            win.separator();
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
