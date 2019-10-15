use xcb;

pub struct Style {
    connection: xcb::Connection,
    color: xcb::Colormap,
    border_width: u8,
    titlebar_width: u8,
}

