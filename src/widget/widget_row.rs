#![allow(unused)]
use crate::{widget::*, window_style::WindowStyle};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct WidgetRow {
    pub widget_holder: WidgetHolder,
    pub font: Option<Font>
}
impl WidgetRow {
    pub fn new(font: Option<Font>) -> Self {
		Self {
            widget_holder: WidgetHolder::new(Vec2::X),
            font,
        }
    }
}

impl Widget for WidgetRow {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn get_type(&self) -> &str {
        "widget_row"
    }

    fn render(&mut self, _: Vec2, delta: Vec2, style: &WindowStyle) {
		self.widget_holder.add_delta_position(delta);
        self.widget_holder.render(style, delta);
    }

    fn update(&mut self, _: Option<&mut dyn Widget>, pos: Vec2) -> Vec2 {
        self.widget_holder.update(pos);

        Vec2::Y * 16.
    }
}

impl WidgetRow {
    pub fn text(&mut self, text: impl ToString) -> &mut Text {
        type T = Text;
        let b = Box::new(T::new(text, self.font.clone()));
        let x = self.widget_holder.add_widget(b);

        if x < self.widget_holder.previous.len() && self.widget_holder.previous.len() > 0 {
            return self.widget_holder.previous[x]
                .as_any()
                .downcast_mut::<T>()
                .unwrap();
        } else {
            return self.widget_holder.widgets[x]
                .as_any()
                .downcast_mut::<T>()
                .unwrap();
        }
    }

    pub fn button(&mut self, text: impl ToString) -> &mut Button {
        type T = Button;
        let b = Box::new(T::new(text, self.font.clone()));
        let x = self.widget_holder.add_widget(b);

        if x < self.widget_holder.previous.len() && self.widget_holder.previous.len() > 0 {
            return self.widget_holder.previous[x]
                .as_any()
                .downcast_mut::<T>()
                .unwrap();
        } else {
            return self.widget_holder.widgets[x]
                .as_any()
                .downcast_mut::<T>()
                .unwrap();
        }
    }
}
