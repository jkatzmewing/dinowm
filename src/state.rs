use crate::xorg::Xorg;

pub struct WmState {
    window_changing: bool,
    screen_w: u16,
    screen_h: u16,
}

impl WmState {
    pub fn new(xorg: &Xorg) -> Self {
        WmState {
            window_changing: false,
            screen_w: xorg.screen.width_in_pixels(),
            screen_h: xorg.screen.height_in_pixels(),
        }
    }

    pub fn start_move_resize(&mut self, resizing: bool) {
        self.window_changing = true;
    }

    pub fn stop_move_resize(&mut self) {
        self.window_changing = false;
    }
}
