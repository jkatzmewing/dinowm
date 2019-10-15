use xcb;

enum BindAction {
    Exec {
        command: &str,
    }
    RaiseWindow,
    LowerWindow,
    MoveWindow,
    ResizeWindow,
}

enum BindType {
    KeyBind {
        key: xcb::Keycode,
    }
    MouseBind {
        button: xcb::Button,
    }
}

struct Binding {
    binding: BindType,
    modifiers: u16,
    action: BindAction,
}

impl Binding {
    pub fn grab(&self, connection: xcb::Connection) {
        match self.BindType {
            BindType::KeyBind => {
                Binding::grab_key(
                    connection,
                    key,
                    modifiers,
                );
            }
            BindType::MouseBind => {
                Binding::grab_button(
                    connection,
                    button,
                    modifiers
                );
            }
        }
    }

    fn grab_key(
        connection: xcb::Connection,
        key: xcb::Keycode,
        modifiers: u16,
    ) {
    }

    fn grab_button(
        connection: xcb::Connection,
        button: xcb::Button,
        modifiers: u16,
    ) {
    }
}
