use firefly_rust::audio::{self, AdsrModulator, SineModulator, SquareModulator};
use firefly_rust::{Point, Size, load_file_buf};

use crate::{audio::Audio, game::animation::Spritesheet, input::Input, remap_color};

pub struct Cutscene {
    sheet: Spritesheet,
    t: u32,
}

impl Cutscene {
    pub fn new() -> Self {
        remap_color!(
             1u8.into() => 0x07,0x07,0x08,
             2u8.into() => 0x33,0x22,0x22,
             3u8.into() => 0x77,0x44,0x33,
             4u8.into() => 0xcc,0x88,0x55,
             5u8.into() => 0x99,0x33,0x11,
             6u8.into() => 0xdd,0x77,0x11,
             7u8.into() => 0xff,0xdd,0x55,
             8u8.into() => 0xff,0xff,0x33,
             9u8.into() => 0x55,0xaa,0x44,
            10u8.into() => 0x11,0x55,0x22,
            11u8.into() => 0x44,0xee,0xbb,
            12u8.into() => 0x33,0x88,0xdd,
            13u8.into() => 0x55,0x44,0xaa,
            14u8.into() => 0x55,0x55,0x77,
            15u8.into() => 0xaa,0xbb,0xbb,
            16u8.into() => 0xff,0xff,0xff,
        );
        Self {
            sheet: Spritesheet::new(
                load_file_buf("intro_cutscene").expect("wow").into(),
                Size::new(240, 160),
            ),
            t: 0,
        }
    }

    pub fn update(&mut self, input: &Input, audio: &mut Audio) {
        self.t += 1;

        if self.t > 60 && input.is_any_pressed() {
            self.t = 1000000;
            return;
        }

        match self.get_frame() {
            8 => {
                if !self.just_changed_frame() {
                    return;
                }
                let num_frames = 33 - 8;
                let sound_duration = num_frames as f32 * (12.0 / 60.0) * 1000.0;
                // start squeeeeeeeze
                audio.play_good_sfx(|parent: &audio::Node<audio::Gain>| {
                    let g = parent.add_gain(0.0);

                    g.modulate(
                        0.0,
                        0.75,
                        AdsrModulator {
                            attack: audio::ms((sound_duration * 0.9) as u32),
                            decay: audio::ms((sound_duration * 0.92) as u32),
                            sustain: audio::ms((sound_duration * 0.95) as u32),
                            sustain_level: 1.0,
                            release: audio::ms((sound_duration * 1.0) as u32),
                        },
                    );

                    let s = g.add_sawtooth(0.0.into(), 0.0);
                    s.modulate(
                        40.0.into(),
                        60.0.into(),
                        SquareModulator {
                            period: audio::ms(200),
                        },
                    );
                    let s2 = g.add_triangle(0.0.into(), 0.2);
                    s2.modulate(20.0.into(), 30.0.into(), SineModulator { freq: 5.0.into() });
                });
            }
            34 => {
                // shots fired
                if !self.just_changed_frame() {
                    return;
                }
                audio.play_bad_sfx(|parent: &audio::Node<audio::Gain>| {
                    parent.add_file("gunshot");
                });
            }
            _ => (),
        }
    }

    pub fn render(&self) {
        self.sheet.render(self.get_frame(), Point::new(0, 0));
    }

    fn get_frame(&self) -> i32 {
        self.t as i32 / 12
    }

    fn just_changed_frame(&self) -> bool {
        self.t % 12 == 0
    }

    pub fn is_done(&self) -> bool {
        let num_frames = 42;
        self.get_frame() >= num_frames
    }
}
