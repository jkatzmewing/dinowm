use crate::xorg::Xorg;

pub struct WmState {
    window_moving: bool,
    window_resizing: bool,
    screen_w: u16,
    screen_h: u16,
}

impl WmState {
    pub fn new(xorg: &Xorg) -> Self {
        WmState {
            window_moving: false,
            window_resizing: false,
            screen_w: xorg.screen.width_in_pixels(),
            screen_h: xorg.screen.height_in_pixels(),
        }
    }

    pub fn start_move(&mut self) {
        self.window_moving = true;
    }

    pub fn start_resize(&mut self) {
        self.window_resizing = true;
    }

    pub fn stop_move_resize(&mut self) {
        self.window_moving = false;
        self.window_resizing = false;
    }

    pub fn is_moving(&self) -> bool {
        self.window_moving
    }

    pub fn is_resizing(&self) -> bool {
        self.window_resizing
    }
}
