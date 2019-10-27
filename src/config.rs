use dirs;
use toml;
use xcb;

use toml::Value;

use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::panic;

use crate::bindings::*;
use crate::style::Style;

fn read_config_file(path: &str) -> io::Result<String> {
    let full_conf_path = dirs::config_dir().unwrap().join(path);
    let mut buffer;
    File::open(path)?.read(&mut buffer)?;

    Ok(buffer)
}

pub fn load_config(path: &str) -> (Style, Vec<Binding>) {
    let config = read_config_file(path).unwrap();
    let doc = config.parse::<Value>().unwrap();

    // Parse the style
    let style = Style {
        border_width: doc["style.border_width"].try_into().unwrap(),
        titlebar_width: doc["style.titlebar_width"].try_into().unwrap(),
    };

    let mut all_bindings = vec![];
    for elem in doc["bindings"] {
        let mut binding: Binding;
        binding.modifiers = parse_mods(elem["mods"]);
        binding.key = parse_key(elem["key"]);
        binding.action = parse_action(elem["action"]);

        all_bindings.push(binding);
    }

    (style, all_bindings)
}

fn parse_mods(input: &str) -> u16 {
}

fn parse_key(input: &str) -> xcb::Keycode {
}

fn parse_action(input: &str) -> BindAction {
}
