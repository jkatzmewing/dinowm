extern crate toml;
extern crate x11_keysymdef;
extern crate xcb;

mod bindings;
mod config;
mod style;
mod windows;
mod xorg;

use bindings::Binding;
use style::Style;
use xorg::Xorg;

const DINOWM_CONFIG_PATH: &str = ".config/dinowm/dinowm.toml";

// TODO fix this to not use a purely relative config path
fn main() {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let xorg = Xorg {
        connection: &conn,
        setup: &setup,
        screen: &screen,
    };
    let (style, bindings) = config::load_config(&xorg, DINOWM_CONFIG_PATH);
    main_loop(&xorg, style, bindings);
}

fn main_loop(xorg: &Xorg, style: Style, bindings: Vec<Binding>) {
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
}
