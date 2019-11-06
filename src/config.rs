use toml;
use x11_keysymdef;
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
        border_width: doc["style"]["border_width"]
            .clone()
            .try_into().
            unwrap(),
        titlebar_width: doc["style"]["titlebar_width"]
            .clone()
            .try_into()
            .unwrap(),
    };

    let mut all_bindings = vec![];
    for elem in doc["bindings"].as_array().unwrap().iter() {
        let binding = Binding {
            modifiers: parse_mods(elem["mods"].as_str().unwrap()),
            input: InputType::Key {
                key: parse_keysym(elem["key"].as_str().unwrap()),
            },
            action: parse_action(elem["action"].as_str().unwrap()),
        };
        all_bindings.push(binding);
    }

    (style, all_bindings)
}

fn parse_mods(input: &str) -> u16 {
    use gdk::ModifierType as Mod;
    let text = input.to_string();
    let mut mods = gdk::ModifierType::empty();
    // TODO add support for Apple command key
    if text.contains("alt") {
        mods.set(Mod::MOD1_MASK, true);
    }
    if text.contains("control") {
        mods.set(Mod::CONTROL_MASK, true);
    }
    if text.contains("shift") {
        mods.set(Mod::SHIFT_MASK, true);
    }
    if text.contains("win") {
        mods.set(Mod::MOD4_MASK, true);
    }

    mods.bits().try_into().unwrap()
}

fn parse_keysym(input: &str) -> xcb::Keysym {
    let record = x11_keysymdef::lookup_by_name(input).unwrap();
    record.keysym.try_into().unwrap()
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
    } else if text == "resize-window" {
        action = BindAction::ResizeWindow;
    } else if text == "move-window" {
        action = BindAction::MoveWindow;
    } else {
        panic!("Config contains an invalid key or mouse binding action");
    }
    
    action
}
