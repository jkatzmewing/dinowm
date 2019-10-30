extern crate toml;
extern crate xcb;

mod bindings;
mod config;
mod style;

const DINOWM_CONFIG_PATH: &str = ".config/dinowm/dinowm.toml";

// TODO fix this to not use a purely relative config path
fn main() {
    let (style, bindings) = config::load_config(DINOWM_CONFIG_PATH);
}
