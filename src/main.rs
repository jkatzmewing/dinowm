extern crate dirs;
extern crate toml;
extern crate x11_keysymdef;
extern crate xcb;

use std::convert::TryInto;

mod bindings;
mod config;
mod style;
mod windows;
#[macro_use]
mod xorg;

use bindings::Binding;
use style::Style;
use xorg::Xorg;

const DINOWM_CONFIG_PATH: &str = "dinowm/dinowm.toml";

// TODO fix this to not use a purely relative config path
fn main() {
    let mut pathbuf = dirs::config_dir().unwrap();
    pathbuf.push(DINOWM_CONFIG_PATH);

    setup_xorg!(xorg);
    let (style, bindings) = config::load_config(&xorg, pathbuf.to_str().unwrap());
    main_loop(&xorg, style, bindings);
}

fn main_loop(xorg: &Xorg, style: Style, bindings: Vec<Binding>) {
    xcb::grab_key(
        xorg.connection,
        true,
        xorg.screen.root(),
        xcb::MOD_MASK_ANY.try_into().unwrap(),
        xcb::GRAB_ANY.try_into().unwrap(),
        xcb::GRAB_MODE_ASYNC.try_into().unwrap(),
        xcb::GRAB_MODE_ASYNC.try_into().unwrap(),
    );
    xcb::grab_button(
        xorg.connection,
        true,
        xorg.screen.root(),
        xcb::EVENT_MASK_BUTTON_PRESS.try_into().unwrap(),
        xcb::GRAB_MODE_ASYNC.try_into().unwrap(),
        xcb::GRAB_MODE_ASYNC.try_into().unwrap(),
        xcb::NONE,
        xcb::NONE,
        xcb::BUTTON_MASK_ANY.try_into().unwrap(),
        xcb::MOD_MASK_ANY.try_into().unwrap(),
    );

    loop {
        if let Some(ev) = xorg.connection.wait_for_event() {
            match ev.response_type() & !0x80 {
                xcb::KEY_PRESS => {
                    let key: &xcb::KeyPressEvent = unsafe { xcb::cast_event(&ev) };
                    bindings::process_key(xorg, key, &bindings);
                }

                xcb::BUTTON_PRESS => {
                    let button: &xcb::ButtonPressEvent = unsafe { xcb::cast_event(&ev) };
                    bindings::process_button(xorg, button, &bindings);
                }

                xcb::CREATE_NOTIFY => {
                    let notify: &xcb::CreateNotifyEvent = unsafe { xcb::cast_event(&ev) };
                    windows::reparent_window(xorg, notify, &style);
                }

                xcb::DESTROY_NOTIFY => {}

                _ => (),
            }
        }
    }

    xcb::ungrab_button(
        xorg.connection,
        xcb::BUTTON_MASK_ANY.try_into().unwrap(),
        xorg.screen.root(),
        xcb::MOD_MASK_ANY.try_into().unwrap(),
    );
    xcb::ungrab_key(
        xorg.connection,
        xcb::GRAB_ANY.try_into().unwrap(),
        xorg.screen.root(),
        xcb::MOD_MASK_ANY.try_into().unwrap(),
    );
}
