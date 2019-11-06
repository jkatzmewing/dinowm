extern crate toml;
extern crate xcb;
extern crate x11_keysymdef;

mod bindings;
mod config;
mod style;
mod windows;

use bindings::Binding;
use style::Style;

const DINOWM_CONFIG_PATH: &str = ".config/dinowm/dinowm.toml";

// TODO fix this to not use a purely relative config path
fn main() {
    let (style, bindings) = config::load_config(DINOWM_CONFIG_PATH);
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    
    main_loop(conn, setup, screen, style, bindings);
}

fn main_loop(
    conn: xcb::Connection,
    setup: xcb::Setup,
    screen: xcb::Screen,
    style: Style,
    bindings: Vec<Binding>
) {
    loop {
        if let Some(ev) = conn.wait_for_event() {
            match ev.response_type() & !0x80 {
                xcb::KEY_PRESS => {
                    bindings::process_key(con, ev, bindings);
                }
                xcb::BUTTON_PRESS => {
                    bindings::process_button(conn, ev, bindings);
                }
                xcb::CREATE_NOTIFY => {
                    windows::reparent_window(conn, ev, style);
                }
                xcb::DESTROY_NOTIFY => {
                }
            }
        }
    }
}

