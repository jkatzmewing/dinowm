use xcb;

use crate::xorg::Xorg;

#[derive(Debug, PartialEq, Eq)]
pub enum BindAction {
    Exec { command: String },
    RaiseWindow,
    LowerWindow,
    MoveWindow,
    ResizeWindow,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InputType {
    Key { key: xcb::Keysym },
    Button { button: xcb::Button },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Binding {
    pub input: InputType,
    pub modifiers: u16,
    pub action: BindAction,
}

pub fn process_key(xorg: &Xorg, ev: &xcb::KeyPressEvent, bindings: &Vec<Binding>) {
    std::unimplemented!();
}

pub fn process_button(xorg: &Xorg, ev: &xcb::ButtonPressEvent, bindings: &Vec<Binding>) {
    std::unimplemented!();
}
