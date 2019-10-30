use toml;
use xcb;

use toml::Value;

use std::convert::TryInto;
use std::fs;

use crate::bindings::*;
use crate::style::Style;

fn read_config_file(path: &str) -> String {
    fs::read_to_string(path).expect("Couldn't read config file")
}

pub fn load_config(path: &str) -> (Style, Vec<Binding>) {
    let config = read_config_file(path);
    let doc = config.parse::<Value>().unwrap();

    // Parse the style
    let style = Style {
        border_width: doc["style"]["border_width"].try_into().unwrap(),
        titlebar_width: doc["style"]["titlebar_width"].try_into().unwrap(),
    };

    let mut all_bindings = vec![];
    for elem in doc["bindings"].as_array().unwrap().iter() {
        let mut binding: Binding;
        binding.modifiers = parse_mods(elem["mods"].as_str().unwrap());
        binding.key = parse_key(elem["key"].as_str().unwrap());
        binding.action = parse_action(elem["action"].as_str().unwrap());

        all_bindings.push(binding);
    }

    (style, all_bindings)
}

fn parse_mods(input: &str) -> u16 {
    let text = input.to_string();
    let mut mods: u32 = 0;
    // TODO add support for Apple command key
    if text.contains("alt") {
        mods |= xcb::MOD_MASK_1;
    }
    if text.contains("control") {
        mods |= xcb::MOD_MASK_CONTROL;
    }
    if text.contains("shift") {
        mods |= xcb::MOD_MASK_SHIFT;
    }
    if text.contains("win") {
        mods |= xcb::MOD_MASK_4;
    }

    mods.try_into().unwrap()
}

fn parse_key(input: &str) -> xcb::Keycode {
}

fn parse_action(input: &str) -> BindAction {
    let text = input.to_string();
    let action: BindAction;
    if text.starts_with("exec:") {
        let text: Vec<&str> = text.splitn(2, ":").collect();
        let cmd = text[1];
        action = BindAction::Exec {
            command: cmd.to_string(),
        };
    } else if text == "raise-window" {
        action = BindAction::RaiseWindow;
    } else if text == "lower-window" {
        action = BindAction::LowerWindow;
    };
    
    action
}
