use macroquad::prelude::*;

#[derive(Clone)]
pub struct WindowStyle {
    pub background: Color,
    pub outline: Color,
    pub selected_outline: Color,
    pub titlebar: Color,
    pub title: Color,

	pub resize_triangle: Color,
	pub minimize_triangle: Color,
	pub close_button: Color,

	pub button_bg_color: Color,
	pub button_text_color: Color,
	pub button_outline_color: Color
}

impl Default for WindowStyle {
    fn default() -> Self {
        Self {
            background: Color::from_hex(0x151617),
            outline: GRAY,
            selected_outline: LIGHTGRAY,
            titlebar: Color::from_rgba(41, 74, 122, 255),
            title: WHITE,
			resize_triangle: Color::from_rgba(41, 150, 255, 75),
			minimize_triangle: Color::from_rgba(255, 255, 255, 210),
			close_button: WHITE,

			button_bg_color: Color::from_hex(0x294a7a),
			button_text_color: WHITE,
			button_outline_color: GRAY,
        }
    }
}
