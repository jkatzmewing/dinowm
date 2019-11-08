use xcb::render::Color;

// TODO implement fonts and colors
pub struct Style {
    pub border_width: u8,
    pub titlebar_height: u8,

    pub titlebar_color_bg: Color,
    pub titlebar_color_fg: Color,

    pub border_color_bg: Color,
    pub border_color_fg: Color,

    pub text_color_bg: Color,
    pub text_color_fg: Color,
}

