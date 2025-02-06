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
            win.set_size((250., screen_height() / 2.));
            bald(win);

            win.row(|row| {
                row.button("[+]");
                row.button("[search]");
                row.button("[delete]");
                row.button("[...]");
            });

            win.separator();

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
                row.button("workspace");
            });

            if t1 {
                win.indent(24., |indent| {
                    indent.row(|row| {
                        if row
                            .button(match t2 {
                                true => "\\/",
                                false => ">",
                            })
                            .clicked
                        {
                            t2 = !t2
                        }
                        row.button("Part1");
                    });

                    if t2 {
                        indent.indent(24., |indent| {
                            for i in 1..3 {
                                indent.button(format!("Object{i}"));
                            }
                        });
                    }

                    indent.row(|row| {
                        if row
                            .button(match t3 {
                                true => "\\/",
                                false => ">",
                            })
                            .clicked
                        {
                            t3 = !t3
                        }
                        row.button("Part2");
                    });

                    if t3 {
                        indent.indent(24., |indent| {
                            for i in 1..3 {
                                indent.button(format!("Object{i}"));
                            }
                        });
                    }
                });
            }
        });

        windows.begin("properties", |win| {
            win.set_position((0., screen_height() / 2.));
            win.set_size((250., screen_height() / 2.));
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

            if win.checkbox("hello", false).checked {
                win.text("hello");
            }

			win.button("abc");
			win.button("abcdefg");
			win.button("abcdefghjkl");
			win.button("abcdefghjklmnopq");
			win.button("abcdefghjklmnopqrstuvxyz");
        });

        windows.begin("output", |win| {
            win.set_position((250., screen_height() - 150.));
            win.set_size((screen_width() - 250., 150.));

            let mut flags = [false, true, false, false, false];

            win.row(|row| {
                flags[0] = row.checkbox("No Titlebar", flags[0]).checked;
                flags[1] = row.checkbox("Resizable", flags[1]).checked;
                flags[2] = row.checkbox("Minimizable", flags[2]).checked;
            });

			win.row(|row| {
				flags[3] = row.checkbox("Closable", flags[3]).checked;
				flags[4] = row.checkbox("Maple", flags[4]).checked;
			});

            win.no_titlebar = flags[0];
            win.resizable = flags[1];
            win.minimizable = flags[2];
            win.closable = flags[3];
			if flags[4] {
				win.set_style(&WindowStyle::maple());
			} else {
				win.set_style(&WindowStyle::default());
			}
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
