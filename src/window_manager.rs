#![allow(unused)]
use macroquad::{
    math::vec2,
    text::{load_ttf_font, Font},
    time::get_time,
};
use std::collections::HashMap;

use crate::window::*;

pub struct WindowManager {
    windows: HashMap<String, Window>,
    font: Option<Font>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            font: None,
        }
    }

    pub async fn set_font(&mut self, path: &str) {
        if let Ok(f) = load_ttf_font(path).await {
            self.font = Some(f);
        } else {
            println!("Failed to load font, reverting to default.")
        }
    }

    pub fn begin(&mut self, id: impl ToString, mut handler: impl FnMut(&mut Window) -> ()) {
        if self.windows.contains_key(&id.to_string()) {
            let win = self.windows.get_mut(&id.to_string()).unwrap();

            WindowManager::handle_window(win, handler);
            win.once = false;
        } else {
            let mut win = Window::new(id.to_string(), self.font.clone());

            WindowManager::handle_window(&mut win, handler);
            self.windows.insert(id.to_string(), win);
        }
    }

    pub fn update_windows(&mut self) {
        let mut windows = self.windows.values_mut().collect::<Vec<&mut Window>>();
        windows.sort_by(|a, b| {
            (get_time() - a.time_since_selected).total_cmp(&(get_time() - b.time_since_selected))
        });

        let mut selected = false;
        for i in windows {
            i.update(selected);
            if i.selected {
                selected = true;
            }
        }
    }

    pub fn render_windows(&mut self) {
        let mut windows = self.windows.values_mut().collect::<Vec<&mut Window>>();
        windows.sort_by(|a, b| {
            (get_time() - b.time_since_selected).total_cmp(&(get_time() - a.time_since_selected))
        });

        for i in windows.iter_mut() {
            i.render();
        }
    }

    pub fn end_windows(&mut self) {
        self.update_windows();
        self.render_windows();
    }

    pub fn handle_window(win: &mut Window, mut handler: impl FnMut(&mut Window) -> ()) {
        win.widget_holder.clear();
        handler(win);
        win.clamp();

        win.widget_holder.update(
            vec2(
                win.rect.x + 5.,
                win.rect.y
                    + match win.no_titlebar {
                        true => 5.,
                        _ => 25.,
                    },
            ),
            win.selected,
        );
    }
}
