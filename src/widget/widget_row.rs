#![allow(unused)]
use crate::{widget::*, window_style::WindowStyle};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct WidgetRow {
    pub widget_holder: WidgetHolder,
    pub font: Option<Font>,
    pub offset: Vec2,
    pub direction: Vec2,
    pub margin: f32,
    pub length: f32,
}
impl WidgetRow {
    pub fn new(direction: Vec2, font: Option<Font>, offset: Vec2, length: f32) -> Self {
        Self {
            widget_holder: WidgetHolder::new(direction),
            font,
            offset,
            direction,
            margin: 5.,
            length,
        }
    }

    pub fn add_widget<T: Widget>(&mut self, object: T) -> &mut T {
        let b = Box::new(object);
        let x = self.widget_holder.add_widget(b);

        if x < self.widget_holder.previous.len() && self.widget_holder.previous.len() > 0 {
            if let Some(x) = self.widget_holder.previous[x].as_any().downcast_mut::<T>() {
                return x;
            } else {
                println!("{}", self.widget_holder.widgets[x].get_type());
                return self.widget_holder.widgets[x]
                    .as_any()
                    .downcast_mut::<T>()
                    .unwrap();
            }
        } else {
            return self.widget_holder.widgets[x]
                .as_any()
                .downcast_mut::<T>()
                .unwrap();
        }
    }
}

impl Widget for WidgetRow {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

	fn equate(&self, other: &mut dyn Widget) -> bool {
		self.get_type() == other.get_type()
	}

    fn get_type(&self) -> &str {
        "widget_row"
    }

    fn render(&mut self, pos: Vec2, delta: Vec2, style: &WindowStyle) {
        if self.offset.x > 0. {
            // VERTICAL LINE
            draw_rectangle(
                (pos.x + self.offset.x - 15.),
                pos.y - 2.,
                2.,
                14.,
                style.indent_color,
            );

            // HORIZONTAL LINE
            draw_rectangle(
                (pos.x + self.offset.x - 13.),
                pos.y + 10.,
                (self.offset.x - 11.).clamp(0., screen_width()),
                2.,
                style.indent_color,
            );
        }

        self.widget_holder.add_delta_position(delta);
        self.widget_holder.render(style, delta);
    }

    fn update(&mut self, other: Option<&mut dyn Widget>, pos: Vec2, selected: bool) -> Vec2 {
        if let Some(other) = other {
            let new = other.as_any().downcast_ref::<Self>().unwrap();
            self.offset = new.offset.clone();
            self.direction = new.direction;
        }

        self.widget_holder.margin = self.margin;
        self.widget_holder.fill_direction = self.direction;
        let pos = self.widget_holder.update(pos + self.offset, selected);

        if self.direction == Vec2::Y {
            Vec2::Y * pos - 5.
        } else {
            Vec2::Y * 16.
        }
    }
}

impl WidgetRow {
    pub fn text(&mut self, text: impl ToString) -> &mut Text {
        self.add_widget(Text::new(text, self.font.clone()))
    }

    pub fn button(&mut self, text: impl ToString) -> &mut Button {
        self.add_widget(Button::new(text, self.font.clone()))
    }

    pub fn separator(&mut self) -> &mut Separator {
        self.add_widget(Separator::new(Vec2::Y, 20.))
    }

    pub fn row(&mut self, mut handler: impl FnMut(&mut WidgetRow)) {
        let x = self.add_widget(WidgetRow::new(
            Vec2::X,
            self.font.clone(),
            Vec2::ZERO,
            self.length - self.offset.x,
        ));
        x.widget_holder.clear();
        handler(x);
    }

    pub fn indent(&mut self, spacing: f32, mut handler: impl FnMut(&mut WidgetRow)) {
        let x = self.add_widget(WidgetRow::new(
            Vec2::Y,
            self.font.clone(),
            Vec2::X * spacing,
            self.length - self.offset.x,
        ));
        x.widget_holder.clear();
        handler(x);
    }

    pub fn tree<R>(
        &mut self,
        text: impl ToString,
        default: bool,
        mut handler: impl FnMut(&mut WidgetRow) -> R,
    ) -> R {
        let x = self.add_widget(Tree::new(
            text,
            self.font.clone(),
            default,
            self.length - self.offset.x,
        ));
        x.widget_row.widget_holder.clear();
        handler(&mut x.widget_row)
    }

    pub fn checkbox(&mut self, text: impl ToString, initial_checked: bool) -> &mut CheckBox {
        self.add_widget(CheckBox::new(text, self.font.clone(), initial_checked))
    }
}
