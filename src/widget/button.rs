use crate::window_style::WindowStyle;

use super::Widget;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Button {
    pub text: String,
    pub hovering: bool,
    pub pressed: bool,
    pub clicked: bool,
    pub font: Option<Font>,
}

impl Button {
    pub fn new(text: impl ToString, font: Option<Font>) -> Self {
        Self {
            text: text.to_string(),
            font,

            hovering: false,
            pressed: false,
            clicked: false,
        }
    }
}

impl Widget for Button {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn get_type(&self) -> &str {
        "button"
    }

    fn render(&mut self, pos: Vec2, _: Vec2, style: &WindowStyle) {
        let dim = measure_text(&self.text, self.font.as_ref(), 16, 1.);
        let btn_color = style.button_bg_color;

        draw_rectangle(
            pos.x,
            pos.y,
            dim.width + 10.,
            20.,
            match (self.hovering, self.pressed) {
                (true, false) => Color::from_vec(btn_color.to_vec() + Vec4::splat(0.1)),
                (_, true) => Color::from_vec(btn_color.to_vec() + Vec4::splat(0.2)),
                (false, false) => btn_color,
            },
        );

        draw_rectangle_lines(
            pos.x,
            pos.y,
            dim.width + 10.,
            20.,
            1.,
            match (self.hovering, self.pressed) {
                (_, true) => WHITE,
                _ => style.button_outline_color,
            },
        );

        draw_text_ex(
            &self.text,
            pos.x.floor() + 5.,
            pos.y.floor() + 14.,
            TextParams {
                font: self.font.as_ref(),
                font_size: match self.font.is_some() {
					true => 12,
					_ => 16
				},
                color: style.button_text_color,
                ..Default::default()
            },
        );
    }

    fn update(&mut self, other: Option<&mut dyn Widget>, pos: Vec2) -> Vec2 {
        if let Some(other) = other {
            let new = other.as_any().downcast_mut::<Self>().unwrap();
            self.text = new.text.clone();
            (*new).clicked = self.clicked;
        }

        let dim = measure_text(&self.text, self.font.as_ref(), 16, 1.);
        let rect = Rect::new(pos.x, pos.y, dim.width + 10., 20.);

        self.clicked = false;
        if rect.contains(mouse_position().into()) {
            self.hovering = true;
            if is_mouse_button_pressed(MouseButton::Left) && !self.pressed {
                self.pressed = true;
            } else if is_mouse_button_released(MouseButton::Left) && self.pressed {
                self.clicked = true;
            }
        } else {
            self.hovering = false;
        }

        if !is_mouse_button_down(MouseButton::Left) {
            self.pressed = false;
        }

        vec2(dim.width + 10., 20.)
    }
}
