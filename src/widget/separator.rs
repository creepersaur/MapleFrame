use crate::window_style::WindowStyle;

use super::Widget;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Separator {
    pub thickness: f32,
    pub color: Color,
    pub direction: Vec2,
    pub length: f32,
}

impl Separator {
    pub fn new(direction: Vec2, length: f32) -> Self {
        Self {
            thickness: 2.,
            color: DARKGRAY,
            direction,
            length,
        }
    }
}

impl Widget for Separator {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

	fn equate(&self, other: &mut dyn Widget) -> bool {
		self.get_type() == other.get_type()
	}

    fn render(&mut self, pos: Vec2, _: Vec2, _: &WindowStyle) {
        if self.direction == Vec2::Y {
            draw_line(
                pos.x + self.thickness / 2.,
                pos.y,
                pos.x + self.thickness / 2.,
                pos.y + self.length,
                self.thickness,
                self.color,
            );
        } else {
            draw_line(
                pos.x,
                pos.y + self.thickness * 3. / 2.,
                pos.x + self.length - 10.,
                pos.y + self.thickness * 3. / 2.,
                self.thickness,
                self.color,
            );
        }
    }

    fn get_type(&self) -> &str {
        "separator"
    }

    fn update(&mut self, other: Option<&mut dyn Widget>, _: Vec2, _: bool) -> Vec2 {
        if let Some(other) = other {
            let new = other.as_any().downcast_ref::<Self>().unwrap();
            self.length = new.length;
        }
		
        vec2(2., 2.)
    }
}
