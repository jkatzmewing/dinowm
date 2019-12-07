use xcb;

use std::collections::HashMap;
use std::process::Command;

use crate::windows;
use crate::xorg::Xorg;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BindAction {
    Exec { command: String },
    RaiseWindow,
    LowerWindow,
    MoveWindow,
    ResizeWindow,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InputType {
    Key { key: xcb::Keycode },
    Button { button: xcb::Button },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Binding {
    pub input: InputType,
    pub modifiers: u16,
}

impl Binding {
    pub fn key(input: xcb::Keycode, mods: u16) -> Self {
        Binding {
            input: InputType::Key { key: input },
            modifiers: mods,
        }
    }

    pub fn button(input: xcb::Button, mods: u16) -> Self {
        Binding {
            input: InputType::Button { button: input },
            modifiers: mods,
        }
    }
}

pub struct BindingsMap {
    map: HashMap<Binding, BindAction>,
}

impl BindingsMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn lookup_key(&self, xorg: &Xorg, key: xcb::Keycode, mods: u16) -> Option<&BindAction> {
        let binding = Binding::key(key, mods);
        self.map.get(&binding)
    }

    pub fn lookup_button(&self, button: xcb::Button, mods: u16) -> Option<&BindAction> {
        let binding = Binding::button(button, mods);
        self.map.get(&binding)
    }

    pub fn add(&mut self, binding: Binding, action: BindAction) {
        self.map.insert(binding, action);
    }
}

pub fn process_key(xorg: &Xorg, ev: &xcb::KeyPressEvent, bindings: &BindingsMap) {
    if let Some(action) = bindings.lookup_key(xorg, ev.detail(), ev.state()) {
        do_action(xorg, action);
    }
}

pub fn process_key_release(xorg: &Xorg, ev: &xcb::KeyReleaseEvent, bindings: &BindingsMap) {
    use BindAction::*;
    if let Some(action) = bindings.lookup_key(xorg, ev.detail(), ev.state()) {
        // Was a move or resize binding just released?
        if *action == MoveWindow || *action == ResizeWindow {
            windows::end_move_resize(xorg);
        }
    }
}

pub fn process_button(xorg: &Xorg, ev: &xcb::ButtonPressEvent, bindings: &BindingsMap) {
    if let Some(action) = bindings.lookup_button(ev.detail(), ev.state()) {
        do_action(xorg, action);
    }
}

fn do_action(xorg: &Xorg, action: &BindAction) {
    match action {
        BindAction::Exec { command } => {
            let args: Vec<&str> = command.split(' ').collect();
            let p = Command::new("sh").arg("-c").args(args).spawn();
            match p {
                Ok(mut child) => match child.wait() {
                    Ok(k) => (),
                    Err(e) => {
                        eprintln!("Failed to wait on child for command: {}", command);
                    }
                },
                Err(e) => {
                    eprintln!("Failed to execute command: {}", e);
                }
            }
        }
        BindAction::RaiseWindow => windows::raise(xorg),
        BindAction::LowerWindow => windows::lower(xorg),
        BindAction::MoveWindow => windows::begin_move_resize(xorg, false),
        BindAction::ResizeWindow => windows::begin_move_resize(xorg, true),
    }
}


