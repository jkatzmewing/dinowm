extern crate gdk;
extern crate toml;

mod bindings;
mod config;
mod style;

use bindings::Binding;
use style::Style;

const DINOWM_CONFIG_PATH: &str = ".config/dinowm/dinowm.toml";

// TODO fix this to not use a purely relative config path
fn main() {
    let (style, bindings) = config::load_config(DINOWM_CONFIG_PATH);
    let display = gdk::Display::get_default();
    match display {
        Some(d) => {
            main_loop(d, style, bindings);
        }
        None => {
            panic!("Could not open GDK display.");
        }
    }
}

fn main_loop(display: gdk::Display, style: Style, bindings: Vec<Binding>) {
    loop {
        let ev = display.get_event();
        match ev {
            Some(e) => {
                if let Some(k) = e.get_keyval() {
                    bindings::process_keyval(k, e.get_state());
                } else if let Some(b) = e.get_button() {
                    bindings::process_button(b, e.get_state());
                }
            }
            None => (),
        }
    }
}
