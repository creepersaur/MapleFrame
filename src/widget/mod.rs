use crate::window_style::WindowStyle;
use macroquad::prelude::*;
use std::any::Any;

pub use button::Button;
pub use checkbox::CheckBox;
pub use separator::Separator;
pub use text::Text;
pub use widget_holder::WidgetHolder;
pub use widget_row::WidgetRow;

mod button;
mod checkbox;
mod separator;
mod text;
mod widget_holder;
mod widget_row;

// WIDGET
pub trait Widget: WidgetClone + Any {
    fn render(&mut self, pos: Vec2, delta: Vec2, style: &WindowStyle);
    fn get_type(&self) -> &str;
    fn as_any(&mut self) -> &mut dyn Any;
    fn update(&mut self, other: Option<&mut dyn Widget>, pos: Vec2, selected: bool) -> Vec2;
}

pub trait WidgetClone {
    fn clone_box(&self) -> Box<dyn Widget>;
}

impl<T> WidgetClone for T
where
    T: 'static + Widget + Clone,
{
    fn clone_box(&self) -> Box<dyn Widget> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Widget> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
