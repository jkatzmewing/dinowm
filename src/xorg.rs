use xcb;

pub struct Xorg<'a> {
    pub connection: &'a xcb::Connection,
    pub setup: &'a xcb::Setup<'a>,
    pub screen: &'a xcb::Screen<'a>,
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
