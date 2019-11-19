use toml;
use x11_keysymdef;
use xcb;
use xcb::render::Color;
use xcb_util::keysyms::KeySymbols;

use toml::Value;

use std::convert::TryInto;
use std::fs;

use crate::bindings::*;
use crate::style::Style;
use crate::xorg::Xorg;

fn read_config_file(path: &str) -> String {
    fs::read_to_string(path).expect("Couldn't read config file")
}

fn get_color(xorg: &Xorg, cmap: xcb::Colormap, name: &str) -> Color {
    let reply = xcb::lookup_color(xorg.connection, cmap, name)
        .get_reply()
        .expect("Config file contains an invalid X11 color name");

    Color::new(
        reply.visual_red(),
        reply.visual_green(),
        reply.visual_blue(),
        255,
    )
}

pub fn load_config(xorg: &Xorg, path: &str) -> (Style, BindingsMap) {
    let config = read_config_file(path);
    let doc = config
        .parse::<Value>()
        .expect("dinowm.toml contains invalid TOML");
    let cmap = xorg.screen.default_colormap();

    let process_color = |e: toml::Value| get_color(&xorg, cmap, e.as_str().unwrap());

    // Parse the style
    let style = Style {
        border_width: doc["style"]["border_width"].clone().try_into().unwrap(),
        titlebar_height: doc["style"]["titlebar_height"].clone().try_into().unwrap(),

        titlebar_color_bg: process_color(doc["style"]["titlebar_color_bg"].clone()),
        titlebar_color_fg: process_color(doc["style"]["titlebar_color_fg"].clone()),

        border_color_bg: process_color(doc["style"]["border_color_bg"].clone()),
        border_color_fg: process_color(doc["style"]["border_color_fg"].clone()),

        text_color_bg: process_color(doc["style"]["text_color_bg"].clone()),
        text_color_fg: process_color(doc["style"]["text_color_fg"].clone()),
    };

    let mut all_bindings = BindingsMap::new();
    for elem in doc["bindings"].as_array().unwrap().iter() {
        let binding: Binding;
        let mods = parse_mods(elem["mods"].clone().try_into().unwrap());
        if let Some(key) = parse_key(&xorg, elem["key"].clone().try_into().unwrap()) {
            binding = Binding::key(key, mods);
        } else if let Some(button) = elem["button"].clone().try_into().unwrap() {
            binding = Binding::button(button, mods);
        } else {
            panic!("Config file contains an invalid binding entry");
        }
        let action = parse_action(elem["action"].clone().as_str().unwrap());

        all_bindings.add(binding.clone(), action.clone());
    }

    (style, all_bindings)
}

fn parse_mods(input: &str) -> u16 {
    let text = input.to_string();
    let mut mods = 0;
    // TODO add support for Apple command key
    if text.contains("A") {
        mods |= xcb::MOD_MASK_1;
    }
    if text.contains("C") {
        mods |= xcb::MOD_MASK_CONTROL;
    }
    if text.contains("S") {
        mods |= xcb::MOD_MASK_SHIFT;
    }
    if text.contains("W") {
        mods |= xcb::MOD_MASK_4;
    }

    mods.try_into().unwrap()
}

fn parse_key(xorg: &Xorg, input: &str) -> Option<xcb::Keycode> {
    let record = x11_keysymdef::lookup_by_name(input).unwrap();
    let keysym = record.keysym.try_into().unwrap();
    let symbols = KeySymbols::new(xorg.connection);

    // TODO figure out if subsequent KeycodeIter values are relevant
    symbols.get_keycode(keysym).next()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mods() {
        let mut mods = 0;
        mods = xcb::MOD_MASK_1 | xcb::MOD_MASK_4| xcb::MOD_MASK_SHIFT | xcb::MOD_MASK_CONTROL;
        assert_eq!(parse_mods("WACS"), mods as u16);
    }

    #[test]
    fn test_parse_action() {
        assert_eq!(parse_action("exec:rofi -show drun"), BindAction::Exec {
            command: "rofi -show drun".to_string(),
        });
        assert_eq!(parse_action("raise-window"), BindAction::RaiseWindow);
        assert_eq!(parse_action("lower-window"), BindAction::LowerWindow);
        assert_eq!(parse_action("resize-window"), BindAction::ResizeWindow);
        assert_eq!(parse_action("move-window"), BindAction::MoveWindow);
    }
}
