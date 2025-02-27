use crate::{widget::Widget, window_style::WindowStyle};
use macroquad::prelude::*;

#[derive(Clone)]
pub struct WidgetHolder {
    pub widgets: Vec<Box<dyn Widget>>,
    pub previous: Vec<Box<dyn Widget>>,
    pub positions: Vec<Vec2>,
    pub fill_direction: Vec2,
    pub margin: f32,
}

impl WidgetHolder {
    pub fn new(fill_direction: Vec2) -> Self {
        Self {
            widgets: vec![],
            previous: vec![],
            positions: vec![],
            fill_direction,
            margin: 5.,
        }
    }

    pub fn clear(&mut self) {
        self.previous = self.widgets.clone();
        self.widgets.clear();
    }

    pub fn update(&mut self, mut pos: Vec2, selected: bool) -> Vec2 {
        let old_pos = pos.clone();

        self.positions.clear();
        let mut i = 0;

        loop {
            if i >= self.widgets.len() {
                break;
            }

            let mut found_match = false;
            if self.previous.len() > 0 {
                for (prev_i, old) in self.previous.iter_mut().enumerate() {
                    if self.widgets[i].equate(&mut **old) {
                        // Found a match, update the widget and remove it from previous
                        self.positions.push(pos);
                        pos += self.fill_direction
                            * old.update(Some(&mut *self.widgets[i]), pos, selected);
                        self.widgets[i] = old.clone();
                        self.previous.remove(prev_i);
                        found_match = true;
                        break;
                    }
                }
            }

            if !found_match {
                // No match found, update the widget with None
                self.positions.push(pos);
                pos += self.fill_direction * self.widgets[i].update(None, pos, selected);
            }

            pos += self.fill_direction * self.margin;
            i += 1;
        }

        pos - old_pos
    }

    pub fn render(&mut self, style: &WindowStyle, delta: Vec2) {
        if self.widgets.len() != self.positions.len() {
            return;
        }
        for i in 0..self.widgets.len() {
            self.widgets[i].render(self.positions[i], delta, style);
        }
    }

    pub fn add_delta_position(&mut self, pos: Vec2) {
        for i in 0..self.positions.len() {
            self.positions[i] += pos;
        }
    }

    pub fn add_widget(&mut self, w: Box<dyn Widget>) -> usize {
        self.widgets.push(w);
        self.widgets.len() - 1
    }
}
