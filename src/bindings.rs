use xcb;

pub enum BindAction {
    Exec {
        command: String,
    },
    RaiseWindow,
    LowerWindow,
    MoveWindow,
    ResizeWindow,
}

pub enum InputType {
    Key {
        key: xcb::Keysym,
    },
    Button {
        button: xcb::Button,
    },
}

pub struct Binding {
    pub input: InputType,
    pub modifiers: u16,
    pub action: BindAction,
}

