use xcb;

use std::convert::TryInto;

use crate::state::WmState;
use crate::style::Style;
use crate::xorg::Xorg;

fn draw_window_frame(xorg: &Xorg, window: xcb::Window, style: &Style) -> xcb::Window {
    let frame = xorg.connection.generate_id();
    xcb::create_window(
        xorg.connection,
        xcb::COPY_FROM_PARENT as u8,
        frame,
        xorg.screen.root(),
        0,
        0, // TODO smarter window placement
        100,
        100, // TODO actually get width/height from reparented window
        0,
        0, // TODO figure out the correct stuff to put here
        xcb::COPY_FROM_PARENT,
        &[],
    );
    frame
}

fn draw_window_titlebar(xorg: &Xorg, window: xcb::Window, style: &Style) -> xcb::Window {
    let borders: u16 = (style.border_width * 2).try_into().unwrap();
    let titlebar = xorg.connection.generate_id();
    xcb::create_window(
        xorg.connection,
        xcb::COPY_FROM_PARENT as u8,
        titlebar,
        xorg.screen.root(),
        style.border_width, // TODO smarter window placement
        style.border_width,
        100 - borders, // TODO get width from reparented window
        100 - borders,
        0,
        0, // TODO figure out the correct stuff to put here
        xcb::COPY_FROM_PARENT,
        &[],
    );
    titlebar
}

pub fn reparent_window(xorg: &Xorg, ev: &xcb::CreateNotifyEvent, wm_state: &WmState, style: &Style) {
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

    xcb::map_window(xorg.connection, frame);
    xcb::map_window(xorg.connection, titlebar);
    xcb::map_window(xorg.connection, ev.window());
}

pub fn raise(xorg: &Xorg) {
    xcb::circulate_window(
        xorg.connection,
        xcb::CIRCULATE_RAISE_LOWEST as u8,
        xorg.screen.root(),
    );
}

pub fn lower(xorg: &Xorg) {
    xcb::circulate_window(
        xorg.connection,
        xcb::CIRCULATE_LOWER_HIGHEST as u8,
        xorg.screen.root(),
    );
}

pub fn move_current(
    xorg: &Xorg,
    ev: &xcb::MotionNotifyEvent,
    old_x: i16,
    old_y: i16,
) {
    std::unimplemented!();
}

pub fn resize_current(
    xorg: &Xorg,
    ev: &xcb::MotionNotifyEvent,
    old_x: i16,
    old_y: i16,
) {
    std::unimplemented!();
}
