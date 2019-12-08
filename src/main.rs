extern crate dirs;
extern crate toml;
extern crate x11_keysymdef;
extern crate xcb;

mod bindings;
mod config;
mod state;
mod style;
mod windows;
#[macro_use]
mod xorg;

use bindings::BindingsMap;
use style::Style;
use xorg::Xorg;

const DINOWM_CONFIG_PATH: &str = "dinowm/dinowm.toml";

// TODO fix this to not use a purely relative config path
fn main() {
    let mut pathbuf = dirs::config_dir().unwrap();
    pathbuf.push(DINOWM_CONFIG_PATH);

    setup_xorg!(xorg);
    let (style, bindings) = config::load_config(&xorg, pathbuf.to_str().unwrap());
    main_loop(&xorg, style, &bindings);
}

fn main_loop(xorg: &Xorg, style: Style, bindings: &BindingsMap) {
    let mut wm_state = state::WmState::new(xorg);

    xorg.set_grabs(); // must be unset with unset_grabs() on Quit or Restart actions

    loop {
        if let Some(ev) = xorg.connection.wait_for_event() {
            match ev.response_type() & !0x80 {
                xcb::KEY_PRESS => {
                    let key: &xcb::KeyPressEvent = unsafe { xcb::cast_event(&ev) };
                    bindings::process_key(xorg, key, &mut wm_state, &bindings);
                }

                xcb::KEY_RELEASE => {
                    let key: &xcb::KeyReleaseEvent = unsafe { xcb::cast_event(&ev) };
                    bindings::process_key_release(xorg, key, &mut wm_state, &bindings);
                }

                xcb::BUTTON_PRESS => {
                    let button: &xcb::ButtonPressEvent = unsafe { xcb::cast_event(&ev) };
                    bindings::process_button(xorg, button, &mut wm_state, &bindings);
                }

                xcb::CREATE_NOTIFY => {
                    let notify: &xcb::CreateNotifyEvent = unsafe { xcb::cast_event(&ev) };
                    windows::reparent_window(xorg, notify, &mut wm_state, &style);
                }

                xcb::DESTROY_NOTIFY => {}

                _ => (),
            }
        }
    }
}


