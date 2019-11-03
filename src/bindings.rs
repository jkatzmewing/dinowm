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
        keyval: u32,
    },
    Button {
        button: u32,
    },
}

pub struct Binding {
    pub input: InputType,
    pub modifiers: u32,
    pub action: BindAction,
}


pub fn process_keyval(k: u32, state: u32) {
}

pub fn process_button(b: u32, state: u32) {

}
