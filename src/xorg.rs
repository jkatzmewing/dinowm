use xcb;

pub struct Xorg<'a> {
    pub connection: &'a xcb::Connection,
    pub setup: &'a xcb::Setup<'a>,
    pub screen: &'a xcb::Screen<'a>,
}
