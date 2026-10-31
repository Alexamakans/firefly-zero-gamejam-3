use firefly_rust::audio::{self, Gain, Node};

pub mod sounds;

/// why? for funsies, limited to a list of 4 field => method mappings due to firefly zero's max 4
/// children rule
macro_rules! define_audio {
    ($($field:ident => $method:ident),+ $(,)?) => {
        pub struct Audio {
            $(
                $field: Node<Gain>,
            )*
            #[allow(unused)]
            music: Node<Gain>,
        }

        impl Audio {
            pub fn new() -> Self {
                let sfx = audio::OUT.add_gain(0.7);
                Self {
                    $(
                        $field: sfx.add_gain(0.25),
                    )*
                    music: audio::OUT.add_gain(0.3),
                }
            }

            $(
                /// If you modulate the parent it can't be unmodulated.
                #[allow(unused)]
                pub fn $method(&mut self, add_sound: impl Fn(&Node<Gain>)) {
                    Self::play(&self.$field, add_sound);
                }
            )*

            /// If you modulate the parent it can't be unmodulated.
            pub fn play(parent: &Node<Gain>, add_sound: impl Fn(&Node<Gain>)) {
                parent.clear();
                parent.reset();
                add_sound(parent);
            }
        }
    }
}

define_audio! {
    generic_sfx => play_generic_sfx,
    player_sfx => play_player_sfx,
    good_sfx => play_good_sfx,
    bad_sfx => play_bad_sfx,
}
