use crate::window_style::WindowStyle;

use super::Widget;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Text {
    pub text: String,
	pub color: Color,
    pub font: Option<Font>,
}

impl Text {
    pub fn new(text: impl ToString, font: Option<Font>) -> Self {
        Self {
            text: text.to_string(),
            font,
			color: WHITE,
        }
    }
}

impl Widget for Text {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn render(&mut self, pos: Vec2, _: Vec2,  _: &WindowStyle) {
        draw_text_ex(
            &self.text,
            pos.x.floor(),
            pos.y.floor() + 14.,
            TextParams {
                font: self.font.as_ref(),
                font_size: match self.font.is_some() {
					true => 12,
					_ => 16
				},
                color: self.color,
                ..Default::default()
            },
        );
    }

    fn get_type(&self) -> &str {
        "text"
    }

    fn update(&mut self, other: Option<&mut dyn Widget>, _: Vec2) -> Vec2 {
		let dim = measure_text(&self.text, self.font.as_ref(), match self.font.is_some() {
			true => 12,
			_ => 16
		}, 1.);

        if let Some(other) = other {
            let new = other.as_any().downcast_ref::<Self>().unwrap();
            self.text = new.text.clone();
        }
        
		vec2(dim.width, 16.)
    }
}
