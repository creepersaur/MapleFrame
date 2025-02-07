use macroquad::prelude::*;
use mapleframe::prelude::*;

#[macroquad::main("MapleFrame")]
async fn main() {
    let mut windows = WindowManager::new();
    windows.set_font("src/segoe_ui.TTF").await;

    let mut t1 = false;
    let mut t2 = false;
    let mut t3 = false;

    loop {
        windows.begin("explorer", |win| {
            win.set_position((0., 0.));
            win.set_size((250., screen_height() / 3.));
            bald(win);

            win.row(|row| {
                row.button("[+]");
                row.button("[search]");
                row.button("[delete]");
                row.button("[...]");
            });

            win.separator();

            win.row(|row| {
                if row.button(if t1 { "\\/" } else { ">" }).clicked {
                    t1 = !t1;
                }
                row.button("workspace");
            });

            if t1 {
                win.indent(24., |indent| {
                    indent.row(|row| {
                        if row.button(if t2 { "\\/" } else { ">" }).clicked {
                            t2 = !t2;
                        }
                        row.button("Part1");
                    });

                    if t2 {
                        indent.indent(24., |indent| {
                            for i in 1..3 {
                                indent.button(format!("Object{}", i)); // Use format! macro consistently
                            }
                        });
                    }

                    indent.row(|row| {
                        if row.button(if t3 { "\\/" } else { ">" }).clicked {
                            t3 = !t3;
                        }
                        row.button("Part2");
                    });

                    if t3 {
                        indent.indent(24., |indent| {
                            for i in 1..3 {
                                indent.button(format!("Object{}", i)); // Use format! macro consistently
                            }
                        });
                    }
                });
            }
        });

        windows.begin("properties", |win| {
            win.set_position((0., screen_height() / 3.));
            win.set_size((250., screen_height() / 1.5));
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

            if win.checkbox("hello", false).value {
                win.text("hello");
            }

            win.tree("Hello world", false, |tree| {
				tree.text("Hello world");
				tree.row(|row| {
					row.text("Hello world");
					row.button("Hello world");
				});
			});
			
            win.tree("Hello world", false, |tree| {
				tree.text("Hello world");
				tree.button("Foo bar");
				tree.tree("Hello world", false, |tree| {
					tree.text("Hello world");
					tree.button("Foo bar");
					tree.tree("Hello world", false, |tree| {
						tree.text("Hello world");
						tree.button("Foo bar");
					});
				});
			});
        });

        windows.begin("output", |win| {
            win.set_position((250., screen_height()));
            win.set_size((screen_width() - 250., 150.));

            let mut flags = [false, true, false, false, false, false];

            win.row(|row| {
                flags[0] = row.checkbox("No Titlebar", flags[0]).value;
                flags[1] = row.checkbox("Resizable", flags[1]).value;
                flags[2] = row.checkbox("Minimizable", flags[2]).value;
            });

            win.row(|row| {
                flags[3] = row.checkbox("Closable", flags[3]).value;
                flags[4] = row.checkbox("Draggable", flags[4]).value;
                flags[5] = row.checkbox("Maple", flags[5]).value;
            });

            win.no_titlebar = flags[0];
            win.resizable = flags[1];
            win.minimizable = flags[2];
            win.closable = flags[3];
            win.draggable = flags[3];
            win.set_style(&if flags[5] {
                WindowStyle::maple()
            } else {
                WindowStyle::default()
            });
        });

        windows.end_windows();
        next_frame().await
    }
}

fn bald(win: &mut Window) {
    win.closable = false;
    win.draggable = false;
    win.minimizable = false;
}
