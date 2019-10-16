use xcb;

macro_rules! handle_key_error {
    ($1: expr, $2: expr, $3: expr) => {
         match $1 {
            Ok(_) => (),
            Err(_) => {
                // TODO convert the keycode and modifiers into human-readable strings
                eprintln!("Failed to ungrab key: keycode {}, modifiers {}", $2, $3);
            }
        }       
    }
}

macro_rules! handle_mouse_error {
    ($1: expr, $2: expr, $3, expr) => {
        match $1 {
            Ok(_) => (),
            Err(_) => {
                // TODO convert the button and modifiers into human-readable strings
                eprintln!("Failed to ungrab mouse: button {}, modifiers {}", $2, $3);
            }
        }
    }
}

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

    pub fn ungrab(&self, connection: xcb::Connection, root: xcb::Window) {
        match self.BindType {
            BindType::KeyBind => {
                let reply = xcb::ungrab_key(
                    connection,
                    key,
                    root,
                    modifiers,
                ).get_reply();
                handle_key_error!(reply, key, modifiers);
            }
            BindType::MouseBind => {
                let reply = xcb::ungrab_button(
                    connection,
                    button,
                    root,
                    modifiers,
                ).get_reply();
                handle_mouse_error!(reply, button, modifiers);
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
        handle_key_error!(reply, key, modifiers);
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
        handle_mouse_error!(reply, button, modifiers);
    }
}
