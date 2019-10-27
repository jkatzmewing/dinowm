extern crate dirs;
extern crate toml;
extern crate xcb;

mod bindings;
mod config;
mod style;

const DINOWM_CONFIG_PATH: &str = "dinowm/dinowm.toml";

fn main() {
    let (style, bindings) = config::load_config(DINOWM_CONFIG_PATH);
}
