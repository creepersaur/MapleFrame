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
    pub button_outline_color: Color,
	pub button_selected_outline: Color,

	pub checkbox_check: Color,
	pub checkbox_checked: Color,
	
	pub indent_color: Color,

	pub tree_open: Color,
	pub tree_closed: Color
}

impl WindowStyle {
    pub fn maple() -> Self {
        Self {
            titlebar: Color::from_hex(0x781D1D),
            checkbox_checked: Color::from_hex(0xFF4444),
            tree_open: Color::from_hex(0xEE4444),
            selected_outline: Color::from_hex(0xff4444),
            resize_triangle: Color::from_hex(0x5C0E0E),
            button_bg_color: Color::from_hex(0x781D1D),

            ..Default::default()
        }
    }
}

impl Default for WindowStyle {
    fn default() -> Self {
        Self {
            background: Color::from_hex(0x151617),
            title: WHITE,
            titlebar: Color::from_rgba(41, 74, 122, 255),
            outline: GRAY,
            selected_outline: LIGHTGRAY,

            resize_triangle: Color::from_rgba(41, 150, 255, 75),
            minimize_triangle: Color::from_rgba(255, 255, 255, 210),
            close_button: WHITE,

            button_bg_color: Color::from_hex(0x294a7a),
            button_text_color: WHITE,
            button_outline_color: DARKGRAY,
            button_selected_outline: WHITE,

			checkbox_checked: Color::from_rgba(70, 140, 210, 255),
			checkbox_check: WHITE,

			indent_color: Color::new(1.,1.,1.,0.1),

			tree_closed: Color::new(1.,1.,1.,0.05),
			tree_open: Color::from_rgba(70, 140, 210, 255),
        }
    }
}
