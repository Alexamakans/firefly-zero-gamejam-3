use firefly_rust::audio::{self, Gain, Node};

pub mod music;

pub struct Audio {
    generic_sfx: Node<Gain>,
    player_sfx: Node<Gain>,
    good_sfx: Node<Gain>,
    bad_sfx: Node<Gain>,
    #[allow(unused)]
    pub music: Node<Gain>,
}
impl Audio {
    pub fn new() -> Self {
        let sfx = audio::OUT.add_gain(0.9);
        Self {
            generic_sfx: sfx.add_gain(0.25),
            player_sfx: sfx.add_gain(0.25),
            good_sfx: sfx.add_gain(0.25),
            bad_sfx: sfx.add_gain(0.65),
            music: audio::OUT.add_gain(0.05),
        }
    }

    #[doc = r" If you modulate the parent it can't be unmodulated."]
    #[allow(unused)]
    pub fn play_generic_sfx(&mut self, add_sound: impl Fn(&Node<Gain>)) {
        Self::play(&self.generic_sfx, add_sound);
    }

    #[doc = r" If you modulate the parent it can't be unmodulated."]
    #[allow(unused)]
    pub fn play_player_sfx(&mut self, add_sound: impl Fn(&Node<Gain>)) {
        Self::play(&self.player_sfx, add_sound);
    }

    #[doc = r" If you modulate the parent it can't be unmodulated."]
    #[allow(unused)]
    pub fn play_good_sfx(&mut self, add_sound: impl Fn(&Node<Gain>)) {
        Self::play(&self.good_sfx, add_sound);
    }

    #[doc = r" If you modulate the parent it can't be unmodulated."]
    #[allow(unused)]
    pub fn play_bad_sfx(&mut self, add_sound: impl Fn(&Node<Gain>)) {
        Self::play(&self.bad_sfx, add_sound);
    }

    #[doc = r" If you modulate the parent it can't be unmodulated."]
    pub fn play(parent: &Node<Gain>, add_sound: impl Fn(&Node<Gain>)) {
        parent.clear();
        parent.reset();
        add_sound(parent);
    }

    pub fn clear(&self) {
        self.generic_sfx.clear();
        self.player_sfx.clear();
        self.good_sfx.clear();
        self.bad_sfx.clear();
        self.music.clear();
    }
}
