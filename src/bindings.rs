use xcb;

use std::convert::TryInto;

pub enum BindAction {
    Exec {
        command: String,
    },
    RaiseWindow,
    LowerWindow,
    MoveWindow,
    ResizeWindow,
}

pub struct Binding {
    pub key: xcb::Keycode,
    pub modifiers: u16,
    pub action: BindAction,
}

impl Binding {
    pub fn grab(&self, connection: xcb::Connection, root: xcb::Window) {
        Binding::grab_key(
            connection,
            root,
            self.key,
            self.modifiers,
        );
    }

    pub fn ungrab(&self, connection: xcb::Connection, root: xcb::Window) {
        xcb::ungrab_key(
            &connection,
            self.key,
            root,
            self.modifiers,
        );
    }

    fn grab_key(
        connection: xcb::Connection,
        root: xcb::Window,
        key: xcb::Keycode,
        modifiers: u16,
    ) {
        xcb::grab_key(
            &connection,
            false,
            root,
            modifiers,
            key,
            xcb::GRAB_MODE_ASYNC.try_into().unwrap(),
            xcb::GRAB_MODE_ASYNC.try_into().unwrap(),
        );
    }
}
