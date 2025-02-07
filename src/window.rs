#![allow(unused)]
use std::any::Any;

use crate::{widget::*, window_style::WindowStyle};
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Window {
    pub title: String,
    pub rect: Rect,
    pub style: WindowStyle,
    pub widget_holder: WidgetHolder,

    pub open: bool,
    pub minimized: bool,
    pub selected: bool,
    pub dragging: bool,
    pub once: bool,
    pub time_since_selected: f64,
    pub font: Option<Font>,

    pub resizable: bool,
    pub closable: bool,
    pub minimizable: bool,
    pub draggable: bool,
    pub no_titlebar: bool,

    close_pressed: bool,
    minimize_pressed: bool,
    resizing: bool,
    delta: Vec2,
    resize_start: (Vec2, Vec2),
}

////////////////////////////////////////
/// MAIN
////////////////////////////////////////

impl Window {
    pub fn new(title: impl ToString, font: Option<Font>) -> Self {
        Self {
            title: title.to_string(),
            style: WindowStyle::default(),
            rect: Rect::new(50., 50., 200., 106.),
            selected: false,
            dragging: false,
            widget_holder: WidgetHolder::new(Vec2::Y),
            time_since_selected: get_time(),
            delta: Vec2::ZERO,
            resize_start: (Vec2::ZERO, Vec2::ZERO),
            font: font,

            open: true,
            minimized: false,
            close_pressed: false,
            minimize_pressed: false,
            resizable: true,
            resizing: false,
            once: true,
            closable: true,
            minimizable: true,
            draggable: true,
            no_titlebar: false,
        }
    }

	pub fn is_once(&mut self) -> bool {
		self.once
	}

    pub fn set_title(&mut self, title: impl ToString) {
        self.title = title.to_string();
    }

    pub fn set_position(&mut self, position: (f32, f32)) {
        self.rect.x = position.0;
        self.rect.y = position.1
    }

    pub fn set_size(&mut self, size: (f32, f32)) {
        self.rect.w = size.0;
        self.rect.h = size.1;
    }

    pub fn set_style(&mut self, style: &WindowStyle) {
        self.style = style.clone();
    }
}

////////////////////////////////////////
/// RENDER
////////////////////////////////////////

impl Window {
    pub fn render(&mut self) {
        if !self.open {
            return;
        }

        self.draw_window();

        if self.minimized {
            return;
        }

        self.draw_widgets();
    }

    pub fn draw_window(&mut self) {
        // Main Body
        if !self.minimized {
            draw_rectangle(
                self.rect.x,
                self.rect.y,
                self.rect.w,
                self.rect.h,
                self.style.background,
            );
        }

        // TitleBar
        if !self.no_titlebar {
            draw_rectangle(
                self.rect.x,
                self.rect.y,
                self.rect.w,
                20.,
                self.style.titlebar,
            );

            // Title
            draw_text_ex(
                &self.title,
                (self.rect.x
                    + match self.minimizable {
                        true => 22.,
                        _ => 5.,
                    })
                .floor(),
                self.rect.y.floor() + 15.,
                TextParams {
                    font: self.font.as_ref(),
                    font_size: match self.font.is_some() {
                        true => 12,
                        _ => 16,
                    },
                    color: self.style.title,
                    ..Default::default()
                },
            );

            // SEPARATOR
            draw_line(
                self.rect.x,
                self.rect.y + 20.,
                self.rect.x + self.rect.w,
                self.rect.y + 20.,
                1.,
                Color::from_vec(self.style.titlebar.to_vec() + Vec4::splat(0.1)),
            );

            self.draw_close_button();
            self.draw_minimize();
        }
        self.draw_resize();

        // Outline
        draw_rectangle_lines(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            match self.minimized {
                true => 20.,
                _ => self.rect.h,
            },
            2.,
            match self.selected {
                true => self.style.selected_outline,
                _ => self.style.outline,
            },
        );
    }

    pub fn draw_widgets(&mut self) {
        self.widget_holder.render(&self.style, self.delta);
    }

    pub fn draw_close_button(&mut self) {
        if !self.closable {
            return;
        }
        let close_rect = Rect::new(self.rect.x + self.rect.w - 20., self.rect.y, 20., 20.);

        // DRAW BACKGROUND
        draw_rectangle(
            self.rect.x + self.rect.w - 20.,
            self.rect.y,
            20.,
            20.,
            match close_rect.contains(mouse_position().into()) {
                true => Color::from_rgba(0, 0, 0, 100),
                _ => Color::from_rgba(0, 0, 0, 50),
            },
        );

        // DRAW 'X'
        draw_line(
            self.rect.x + self.rect.w - 16.,
            self.rect.y + 4.,
            self.rect.x + self.rect.w - 4.,
            self.rect.y + 16.,
            2.,
            self.style.close_button,
        );

        draw_line(
            self.rect.x + self.rect.w - 4.,
            self.rect.y + 4.,
            self.rect.x + self.rect.w - 16.,
            self.rect.y + 16.,
            2.,
            self.style.close_button,
        );
    }

    pub fn draw_minimize(&mut self) {
        if !self.minimizable {
            return;
        }
        let minimize_rect = Rect::new(self.rect.x + 5., self.rect.y + 5., 10., 10.);

        if self.minimized {
            draw_triangle(
                vec2(self.rect.x + 7., self.rect.y + 5.),
                vec2(self.rect.x + 17., self.rect.y + 10.),
                vec2(self.rect.x + 7., self.rect.y + 15.),
                match minimize_rect.contains(mouse_position().into()) {
                    true => {
                        Color::from_vec(self.style.minimize_triangle.to_vec() + Vec4::splat(0.2))
                    }
                    _ => self.style.minimize_triangle,
                },
            )
        } else {
            draw_triangle(
                vec2(self.rect.x + 7., self.rect.y + 7.),
                vec2(self.rect.x + 17., self.rect.y + 7.),
                vec2(self.rect.x + 12., self.rect.y + 15.),
                match minimize_rect.contains(mouse_position().into()) {
                    true => {
                        Color::from_vec(self.style.minimize_triangle.to_vec() + Vec4::splat(0.2))
                    }
                    _ => self.style.minimize_triangle,
                },
            )
        }
    }

    pub fn draw_resize(&mut self) {
        if !self.resizable || self.minimized {
            return;
        }

        let resize_rect = Rect::new(
            self.rect.x + self.rect.w - 15.,
            self.rect.y + self.rect.h - 15.,
            15.,
            15.,
        );

        draw_triangle(
            vec2(self.rect.x + self.rect.w, self.rect.y + self.rect.h - 20.),
            vec2(self.rect.x + self.rect.w, self.rect.y + self.rect.h),
            vec2(self.rect.x + self.rect.w - 20., self.rect.y + self.rect.h),
            match resize_rect.contains(mouse_position().into()) {
                true => Color::from_vec(self.style.resize_triangle.to_vec() + Vec4::splat(0.1)),
                _ => self.style.resize_triangle,
            },
        );
    }
}

////////////////////////////////////////
/// UPDATE
////////////////////////////////////////

impl Window {
    pub fn update(&mut self, selected: bool) {
        self.delta = vec2(self.rect.x, self.rect.y);

        if selected {
            self.selected = false;
            self.delta = vec2(self.rect.x, self.rect.y) - self.delta;
            self.widget_holder.add_delta_position(self.delta);
            return;
        }

        self.handle_selected();

        if !self.no_titlebar {
            self.handle_close();
            self.handle_minimize();
            self.handle_dragging();
        }

        self.handle_resize();
        self.clamp();

        self.delta = vec2(self.rect.x, self.rect.y) - self.delta;
        self.widget_holder.add_delta_position(self.delta);
    }

    pub fn handle_selected(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let mut rect = Rect::from(self.rect);
            if self.minimized {
                rect.h = 20.;
            }
            self.selected = rect.contains(mouse_position().into())
        }

        if self.selected {
            self.time_since_selected = get_time();
        }
    }

    pub fn handle_dragging(&mut self) {
        if !self.draggable {
            self.dragging = false;
            return;
        }

        let touching_titlebar = Rect::new(self.rect.x, self.rect.y, self.rect.w - 20., 20.)
            .contains(mouse_position().into());

        if is_mouse_button_pressed(MouseButton::Left) && touching_titlebar {
            self.dragging = true;
        }
        if is_mouse_button_released(MouseButton::Left) {
            self.dragging = false;
        }

        if self.dragging {
            self.rect.x -= mouse_delta_position().x * screen_width() / 2.;
            self.rect.y -= mouse_delta_position().y * screen_height() / 2.;
        }
    }

    pub fn clamp(&mut self) {
        self.rect.x = clamp_f32(self.rect.x, 0., screen_width() - self.rect.w);
        self.rect.y = clamp_f32(self.rect.y, 0., screen_height() - match self.minimized {
			true => 20.,
			_ => self.rect.h
		});
    }

    pub fn handle_close(&mut self) {
        if !self.selected || !self.closable {
            return;
        }
        let close_rect = Rect::new(self.rect.x + self.rect.w - 20., self.rect.y, 20., 20.);

        if close_rect.contains(mouse_position().into()) {
            if is_mouse_button_pressed(MouseButton::Left) {
                self.close_pressed = true;
            } else if is_mouse_button_released(MouseButton::Left) && self.close_pressed {
                self.open = false;
            }
        }
    }

    pub fn handle_minimize(&mut self) {
        if !self.selected || !self.minimizable {
            return;
        }
        let minimize_rect = Rect::new(self.rect.x + 5., self.rect.y + 5., 10., 10.);

        if minimize_rect.contains(mouse_position().into()) {
            if is_mouse_button_pressed(MouseButton::Left) {
                self.minimize_pressed = true;
            } else if is_mouse_button_released(MouseButton::Left) && self.minimize_pressed {
                self.minimized = !self.minimized;
            }
        }
    }

    pub fn handle_resize(&mut self) {
        if !self.resizable || self.minimized || !self.selected {
            self.resizing = false;
            return;
        }

        let resize_rect = Rect::new(
            self.rect.x + self.rect.w - 15.,
            self.rect.y + self.rect.h - 15.,
            15.,
            15.,
        );

        if resize_rect.contains(mouse_position().into()) {
            if is_mouse_button_pressed(MouseButton::Left) {
                self.resizing = true;
                self.resize_start = (vec2(self.rect.w, self.rect.h), mouse_position().into())
            }
        }
        if is_mouse_button_released(MouseButton::Left) {
            self.resizing = false;
        }

        if self.resizing {
            let delta = vec2(mouse_position().0, mouse_position().1) - self.resize_start.1;
            self.rect.w = self.resize_start.0.x + delta.x;
            self.rect.h = self.resize_start.0.y + delta.y;

            self.rect.w = clamp_f32(self.rect.w, 40., screen_width());
            self.rect.h = clamp_f32(self.rect.h, 20., screen_height());
        }
    }
}

////////////////////////////////////////
/// WIDGETS
////////////////////////////////////////

impl Window {
    pub fn add_widget<T: Widget>(&mut self, object: T) -> &mut T {
        let b = Box::new(object);
        let x = self.widget_holder.add_widget(b);

        if x < self.widget_holder.previous.len() && self.widget_holder.previous.len() > 0 {
            if let Some(x) = self.widget_holder.previous[x].as_any().downcast_mut::<T>() {
                return x;
            } else {
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

    pub fn text(&mut self, text: impl ToString) -> &mut Text {
        self.add_widget(Text::new(text, self.font.clone()))
    }

    pub fn button(&mut self, text: impl ToString) -> &mut Button {
        self.add_widget(Button::new(text, self.font.clone()))
    }

	pub fn checkbox(&mut self, text: impl ToString, initial_checked: bool) -> &mut CheckBox {
        self.add_widget(CheckBox::new(text, self.font.clone(), initial_checked))
    }

    pub fn separator(&mut self) -> &mut Separator {
        self.add_widget(Separator::new(Vec2::X, self.rect.w))
    }

    pub fn row<R>(&mut self, mut handler: impl FnMut(&mut WidgetRow) -> R) -> R {
        let x = self.add_widget(WidgetRow::new(Vec2::X, self.font.clone(), Vec2::ZERO));
        x.widget_holder.clear();
        handler(x)
    }

    pub fn indent<R>(&mut self, spacing: f32, mut handler: impl FnMut(&mut WidgetRow) -> R) -> R {
        let x = self.add_widget(WidgetRow::new(
            Vec2::Y,
            self.font.clone(),
            Vec2::X * spacing,
        ));
        x.widget_holder.clear();
        handler(x)
    }

	pub fn tree<R>(&mut self, text: impl ToString, default: bool, mut handler: impl FnMut(&mut WidgetRow) -> R) -> R {
        let x = self.add_widget(Tree::new(text, self.font.clone(), default, self.rect.w));
        x.widget_row.widget_holder.clear();
        handler(&mut x.widget_row)
    }
}

////////////////////////////////////////
/// EXTRA
////////////////////////////////////////

fn clamp_f32(num: f32, min: f32, max: f32) -> f32 {
    if num < min {
        min
    } else if num > max {
        max
    } else {
        num
    }
}
