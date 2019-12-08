use xcb;

use std::convert::TryInto;

pub struct Xorg<'a> {
    pub connection: &'a xcb::Connection,
    pub setup: &'a xcb::Setup<'a>,
    pub screen: &'a xcb::Screen<'a>,
}

impl Xorg<'_> {
    pub fn set_grabs(&self) {
        xcb::grab_key(
            self.connection,
            true,
            self.screen.root(),
            xcb::MOD_MASK_ANY.try_into().unwrap(),
            xcb::GRAB_ANY.try_into().unwrap(),
            xcb::GRAB_MODE_ASYNC.try_into().unwrap(),
            xcb::GRAB_MODE_ASYNC.try_into().unwrap(),
        );
        xcb::grab_button(
            self.connection,
            true,
            self.screen.root(),
            xcb::EVENT_MASK_BUTTON_PRESS.try_into().unwrap(),
            xcb::GRAB_MODE_ASYNC.try_into().unwrap(),
            xcb::GRAB_MODE_ASYNC.try_into().unwrap(),
            xcb::NONE,
            xcb::NONE,
            xcb::BUTTON_MASK_ANY.try_into().unwrap(),
            xcb::MOD_MASK_ANY.try_into().unwrap(),
        );
    }

    pub fn unset_grabs(&self) {
        xcb::ungrab_button(
            self.connection,
            xcb::BUTTON_MASK_ANY.try_into().unwrap(),
            self.screen.root(),
            xcb::MOD_MASK_ANY.try_into().unwrap(),
        );
        xcb::ungrab_key(
            self.connection,
            xcb::GRAB_ANY.try_into().unwrap(),
            self.screen.root(),
            xcb::MOD_MASK_ANY.try_into().unwrap(),
        );
    }
}

macro_rules! setup_xorg {
    ($x: ident) => {
        let (conn, screen_num) =
            xcb::Connection::connect(None).expect("Could not connect to X server");
        let setup = conn.get_setup();
        let screen = setup
            .roots()
            .nth(screen_num as usize)
            .expect("Could not get default screen");

        let $x: Xorg = Xorg {
            connection: &conn,
            setup: &setup,
            screen: &screen,
        };
    };
}


