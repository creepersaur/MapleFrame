use crate::window_style::WindowStyle;

use super::Widget;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Indent {
    pub spacing: f32,
}

impl Indent {
    pub fn new(spacing: f32) -> Self {
        Self {
            spacing,
        }
    }
}

impl Widget for Indent {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn render(&mut self, _: Vec2, _: Vec2,  _: &WindowStyle) {
        
    }

    fn get_type(&self) -> &str {
        "indent"
    }

    fn update(&mut self, other: Option<&mut dyn Widget>, _: Vec2, _: bool) -> Vec2 {
        if let Some(other) = other {
            let new = other.as_any().downcast_ref::<Self>().unwrap();
            self.spacing = new.spacing;
        }
        
		vec2(self.spacing, 20.)
    }
}
