use firefly_rust::{Buttons, Pad, Peer, read_buttons, read_pad};

pub struct Input {
    pub old: Buttons,
    pub pressed: Buttons,
    pub held: Buttons,
    pub released: Buttons,
    pub pad: Option<Pad>,
}

impl Input {
    pub fn update(&mut self, peer: Peer) {
        let buttons = read_buttons(peer);
        self.pressed = buttons.just_pressed(&self.old);
        self.held = buttons.held(&self.old);
        self.released = buttons.just_released(&self.old);
        self.old = buttons;
        self.pad = read_pad(peer);
    }
}
