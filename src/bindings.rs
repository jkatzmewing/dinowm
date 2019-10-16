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
    pub fn grab(&self, connection: xcb::Connection, root: xcb::Window) {
        match self.BindType {
            BindType::KeyBind => {
                Binding::grab_key(
                    connection,
                    root,
                    key,
                    modifiers,
                );
            }
            BindType::MouseBind => {
                Binding::grab_button(
                    connection,
                    root,
                    button,
                    modifiers
                );
            }
        }
    }

    fn grab_key(
        connection: xcb::Connection,
        root: xcb::Window,
        key: xcb::Keycode,
        modifiers: u16,
    ) {
        let reply = xcb::grab_key(
            connection,
            false,
            root,
            modifiers,
            key,
            xcb::GRAB_MODE_ASYNC,
            xcb::GRAB_MODE_ASYNC,
        ).get_reply();
        match reply {
            Ok(_) => (),
            Err(_) => {
                // TODO convert the keycode and modifiers into human-readable strings
                eprintln!("Failed to grab key: keycode {}, modifiers {}", key, modifiers);
            }
        }
    }

    fn grab_button(
        connection: xcb::Connection,
        root: xcb::Window,
        button: xcb::Button,
        modifiers: u16,
    ) {
        let reply = xcb::grab_button(
            connection,
            false,
            root,
            xcb::EVENT_MASK_BUTTON_PRESS,
            xcb::GRAB_MODE_ASYNC,
            xcb::GRAB_MODE_ASYNC,
            NONE,
            NONE,
            modifiers,

        ).get_reply();
        match reply {
            Ok(_) => (),
            Err(_) => {
                // TODO convert modifiers into human-readable strings
                eprintln!("Failed to grab mouse: button {}, modifiers {}", button, modifiers);
            }
        }
    }
}
