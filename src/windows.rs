use xcb;

use crate::style::Style;
use crate::xorg::Xorg;

fn draw_window_frame(xorg: &Xorg, window: xcb::Window, style: &Style) -> xcb::Window {
    std::unimplemented!()
}

fn draw_window_titlebar(xorg: &Xorg, window: xcb::Window, style: &Style) -> xcb::Window {
    std::unimplemented!()
}

pub fn reparent_window(xorg: &Xorg, ev: &xcb::CreateNotifyEvent, style: &Style) {
    let titlebar = draw_window_titlebar(xorg, ev.window(), style);
    let frame = draw_window_frame(xorg, ev.window(), style);
    xcb::reparent_window(
        xorg.connection,
        titlebar,
        frame,
        style.border_width,
        style.border_width,
    );
    xcb::reparent_window(
        xorg.connection,
        ev.window(),
        frame,
        style.border_width,
        (style.border_width * 2) + style.titlebar_height,
    );
}
