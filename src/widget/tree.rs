use crate::window_style::WindowStyle;

use super::{Widget, WidgetRow};
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Tree {
    pub text: String,
    pub hovering: bool,
    pub pressed: bool,
    pub clicked: bool,
    pub open: bool,
    pub font: Option<Font>,
    pub widget_row: WidgetRow,
    pub length: f32,

    tried_clicking: bool,
}

impl Tree {
    pub fn new(text: impl ToString, font: Option<Font>, default: bool, length: f32) -> Self {
        Self {
            text: text.to_string(),
            font: font.clone(),
            tried_clicking: false,
            length,

            hovering: false,
            pressed: false,
            clicked: false,
            open: default,
            widget_row: WidgetRow::new(Vec2::Y, font, Vec2::X * 24., length),
        }
    }
}

impl Widget for Tree {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn get_type(&self) -> &str {
        "tree"
    }

    fn render(&mut self, pos: Vec2, delta: Vec2, style: &WindowStyle) {
        // DRAW ROW
        if self.open {
            self.widget_row.render(pos + Vec2::Y * 20., delta, style);
        }

        let btn_color = style.tree_closed;
        let open_color = style.tree_open;

        let bg_color = match self.open {
            true => open_color,
            _ => match (self.hovering, self.pressed) {
                (true, false) => Color::from_vec(btn_color.to_vec() + Vec4::splat(0.1)),
                (_, true) => Color::from_vec(btn_color.to_vec() + Vec4::splat(0.2)),
                (false, false) => btn_color,
            },
        };

        draw_rectangle(pos.x, pos.y, self.length - 10., 20., bg_color);

        draw_rectangle_lines(
            pos.x,
            pos.y,
            self.length - 10.,
            20.,
            1.,
            match (self.hovering, self.pressed) {
                (_, true) => WHITE,
                _ => style.button_outline_color,
            },
        );

        draw_text_ex(
            &self.text,
            pos.x + 20.,
            pos.y.floor() + 14.,
            TextParams {
                font: self.font.as_ref(),
                font_size: match self.font.is_some() {
                    true => 12,
                    _ => 16,
                },
                color: style.button_text_color,
                ..Default::default()
            },
        );

        // MINIMIZE TRIANGLE
        draw_minimize(self.open, pos, style);
    }

    fn update(&mut self, other: Option<&mut dyn Widget>, pos: Vec2, selected: bool) -> Vec2 {
        let mut height = 20.;
        if let Some(other) = other {
            let new = other.as_any().downcast_mut::<Self>().unwrap();
            self.text = new.text.clone();
			new.widget_row = self.widget_row.clone();

            // UPDATE ROW
            if self.open {
                height += self
                    .widget_row
                    .update(Some(&mut new.widget_row), pos + Vec2::Y * 20., selected)
                    .y;
            }
        }

        let rect = Rect::new(pos.x, pos.y, self.length - 10., 20.);

        if self.tried_clicking && selected {
            self.tried_clicking = false;
            self.pressed = true;
        }

        self.clicked = false;
        if rect.contains(mouse_position().into()) {
            self.hovering = true;
            if is_mouse_button_pressed(MouseButton::Left) && !self.pressed {
                if selected {
                    self.pressed = true;
                    self.tried_clicking = false;
                } else {
                    self.tried_clicking = true;
                }
            } else if is_mouse_button_released(MouseButton::Left) && self.pressed && selected {
                self.clicked = true;
                self.open = !self.open;
            }
        } else {
            self.hovering = false;
        }

        if !is_mouse_button_down(MouseButton::Left) {
            self.pressed = false;
        }

        vec2(self.length - 10., height)
    }
}

pub fn draw_minimize(open: bool, pos: Vec2, style: &WindowStyle) {
    let minimize_rect = Rect::new(pos.x + 5., pos.y + 5., 10., 10.);

    if !open {
        draw_triangle(
            vec2(pos.x + 7., pos.y + 5.),
            vec2(pos.x + 17., pos.y + 10.),
            vec2(pos.x + 7., pos.y + 15.),
            match minimize_rect.contains(mouse_position().into()) {
                true => Color::from_vec(style.minimize_triangle.to_vec() + Vec4::splat(0.2)),
                _ => style.minimize_triangle,
            },
        )
    } else {
        draw_triangle(
            vec2(pos.x + 7., pos.y + 7.),
            vec2(pos.x + 17., pos.y + 7.),
            vec2(pos.x + 12., pos.y + 15.),
            match minimize_rect.contains(mouse_position().into()) {
                true => Color::from_vec(style.minimize_triangle.to_vec() + Vec4::splat(0.2)),
                _ => style.minimize_triangle,
            },
        )
    }
}
