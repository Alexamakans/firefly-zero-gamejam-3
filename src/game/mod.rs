mod animation;
mod background;
mod camera;
pub mod cutscene;
mod goat;
mod hud;
mod macros;
mod parallax;
mod platform;
mod position;

use crate::audio::music::MusicPlayer;
use crate::game::background::Background;
use crate::game::camera::Camera;
use crate::game::cutscene::Cutscene;
use crate::game::parallax::{Item, Parallax};
use crate::game::platform::Platform;
use crate::game::position::Position;
use crate::input::Input;
use crate::remap_color;
use crate::{audio::Audio, game::hud::HUD};
use firefly_rust::audio::{AdsrModulator, LinearModulator};
use firefly_rust::{
    Color, FontBuf, Peer, Point, audio, clear_screen, draw_text, get_random, load_file_buf,
    log_debug, read_buttons, read_pad,
};
use goat::Goat;

pub struct Game {
    cutscene: Option<Cutscene>,

    audio: Audio,
    input: Input,

    camera: Camera,
    camera_goat_offset: Position,

    goat: Goat,
    platforms: [Platform; 4],

    parallax: Parallax,
    background: Background,
    ticks_elapsed: u32,
    dead_ticks_elapsed: u32,

    music_player: MusicPlayer,

    font: FontBuf,
    hud: HUD,
    score: u32,
}

impl Game {
    pub fn new(play_cutscene: bool) -> Self {
        let old = read_buttons(Peer::COMBINED);
        let offscreen = Position::new(1000.0, 1000.0);
        let goat_pos = Position::new(0.0, 80.0);
        let mut platforms = [
            Platform::new(goat_pos + Position::new(0.0, 50.0), 200),
            Platform::new(offscreen, 1),
            Platform::new(offscreen, 1),
            Platform::new(offscreen, 1),
        ];

        Self::config_next_platform(&mut platforms, 0, 1);
        Self::config_next_platform(&mut platforms, 1, 2);
        Self::config_next_platform(&mut platforms, 2, 3);

        let camera = Camera::new(goat_pos);
        let audio = Audio::new();
        let music_player = MusicPlayer::new(&audio);
        Self {
            cutscene: if play_cutscene {
                Some(Cutscene::new())
            } else {
                None
            },
            audio,
            input: Input {
                old,
                pressed: old,
                held: old,
                released: old,
                pad: read_pad(Peer::COMBINED),
            },

            camera,
            // world_width / 2 - goat_width / 2 for x
            camera_goat_offset: Position::new(-120.0 + 12.5, -60.0),

            goat: Goat::new(goat_pos),
            platforms,

            parallax: Parallax::new(),
            background: Background::new(),

            ticks_elapsed: 0,
            dead_ticks_elapsed: 0,

            music_player,

            hud: HUD::new(),
            font: load_file_buf("default_font").expect("woah").into(),
            score: 0,
        }
    }

    pub fn update(&mut self) {
        if self.cutscene.is_some() {
            let cutscene = self.cutscene.as_mut().unwrap();
            self.input.update(Peer::COMBINED);
            cutscene.update(&self.input, &mut self.audio);
            if !cutscene.is_done() {
                return;
            }
            self.audio.clear();
            self.music_player.reset(&mut self.audio);
            self.cutscene = None;
            // back to normal palette
            remap_color!(
                 1u8.into() => 0x1a,0x1c,0x2c,
                 2u8.into() => 0x5d,0x27,0x5d,
                 3u8.into() => 0xb1,0x3e,0x53,
                 4u8.into() => 0xef,0x7d,0x57,
                 5u8.into() => 0xff,0xcd,0x75,
                 6u8.into() => 0xa7,0xf0,0x70,
                 7u8.into() => 0x38,0xb7,0x64,
                 8u8.into() => 0x25,0x71,0x79,
                 9u8.into() => 0x29,0x36,0x6f,
                10u8.into() => 0x3b,0x5d,0xc9,
                11u8.into() => 0x41,0xa6,0xf6,
                12u8.into() => 0x73,0xef,0xf7,
                13u8.into() => 0xf4,0xf4,0xf4,
                14u8.into() => 0x94,0xb0,0xc2,
                15u8.into() => 0x56,0x6c,0x86,
                16u8.into() => 0x33,0x3c,0x57,
            );
            remap_color!(Color::Blue => 46, 48, 64);
            self.background.set_palette();
        }
        self.input.update(Peer::COMBINED);

        for p in self.platforms {
            if self.goat.pos.y > p.pos.y + 1000.0 {
                self.goat.kill(&mut self.audio);
            }
        }

        if self.goat.dead {
            self.dead_update();
            return;
        }

        self.music_player.update();

        for i in 0..self.platforms.len() {
            let x = self.platforms[i].pos.x - self.camera.pos.x;
            if (x + self.platforms[i].get_width() as f32) < 0.0 {
                let prev_index = if i == 0 {
                    self.platforms.len() - 1
                } else {
                    i - 1
                };
                Self::config_next_platform(&mut self.platforms, prev_index, i);
                break;
            }
        }

        let old_goat_pos = self.goat.pos;
        // collision - platforms
        let goat_left = self.goat.pos + Position::new(0.0, self.goat.bounds.y);
        let goat_right = self.goat.pos + self.goat.bounds;
        let mut collides: bool = false;
        for p in &self.platforms {
            if p.collision(&goat_left) || p.collision(&goat_right) {
                self.goat.collides_y = p.pos.y - self.goat.bounds.y;
                self.goat.collided_platform = *p;
                collides = true;
                break;
            }
        }
        self.goat.collides_platform = collides;

        self.goat.update(&self.input, &mut self.audio);

        let rightmost_platform_index = {
            let mut rightest = -2000.0;
            let mut rightest_i = 0;
            for i in 0..self.platforms.len() {
                let v = &self.platforms[i];
                if v.pos.x > rightest {
                    rightest = v.pos.x;
                    rightest_i = i;
                }
            }
            rightest_i
        };
        self.parallax
            .update(&self.camera, &self.platforms[rightmost_platform_index]);

        if self.goat.dash.is_dashing() {
            self.camera.pos.lerp(
                0.09,
                self.goat.pos + self.camera_goat_offset - self.goat.vel * 0.5,
            );
        } else {
            self.camera.pos.lerp(
                0.12,
                self.goat.pos + self.camera_goat_offset + self.goat.vel * 15.0,
            );
        }

        self.background.update(&self.camera);

        if self.goat.pos.x > old_goat_pos.x {
            self.score += (self.goat.pos.x - old_goat_pos.x) as u32;
        }
        self.music_player.update();
        self.ticks_elapsed += 1;

        let mut make_sound = false;
        self.parallax
            .on_collision(self.goat.pos, 15.0, &mut |item: &mut Item| {
                match item.kind {
                    parallax::ItemKind::Pumpkin => (),
                    parallax::ItemKind::Witch => {
                        self.score += 500;
                        make_sound = true;
                        item.dead_ticks_elapsed = 1;
                    }
                    parallax::ItemKind::Bat => {
                        self.score += 250;
                        make_sound = true;
                        item.dead_ticks_elapsed = 1;
                    }
                    parallax::ItemKind::Tree => (),
                    parallax::ItemKind::Portal => {
                        if !self.goat.dash.is_dashing() {
                            self.goat.kill(&mut self.audio);
                        } else {
                            self.score += 1000;
                            make_sound = true;
                            item.dead_ticks_elapsed = 1;
                        }
                    }
                };
            });
        if make_sound {
            self.play_points_sound();
        }
    }

    pub fn dead_update(&mut self) {
        self.dead_ticks_elapsed += 1;
        if self.input.is_any_pressed() && self.dead_ticks_elapsed > 60 {
            self.dead_ticks_elapsed = 0;
            self.reset();
        }
    }

    pub fn render(&self) {
        if self.cutscene.is_some() {
            self.cutscene.as_ref().unwrap().render();
            return;
        }
        clear_screen(Color::Blue);

        self.background.render();

        self.parallax.render(&self.camera);
        for p in self.platforms {
            p.render(&self.camera);
        }
        for p in self.platforms {
            p.render_indicator(&self.camera);
        }
        self.goat.render(&self.camera);
        self.hud.render(self.score, &self.goat);

        if self.goat.dead {
            self.dead_render();
        }
    }

    pub fn dead_render(&self) {
        self.goat.render(&self.camera);
        self.hud.render(self.score, &self.goat);
        let s = "press any button to play again";
        let w = (6 * s.len()) as i32;
        let h = 9;
        draw_text(
            s,
            &self.font,
            Point::new(120 - w / 2, 80 - h / 2),
            Color::White,
        );
    }

    pub fn play_points_sound(&mut self) {
        self.audio
            .play_good_sfx(|parent: &audio::Node<audio::Gain>| {
                let r1 = MusicPlayer::get_random_f32_01();
                let r2 = MusicPlayer::get_random_f32_01();
                let r3 = MusicPlayer::get_random_f32_01();
                let g = parent.add_gain(0.0);

                g.modulate(
                    0.0,
                    1.0,
                    AdsrModulator {
                        attack: audio::ms(50),
                        decay: audio::ms(100),
                        sustain: audio::ms(200),
                        sustain_level: 0.2,
                        release: audio::ms(400),
                    },
                );

                let s = g.add_sine(0.0.into(), 0.0);
                s.modulate(
                    (900.0 + 200.0 * r1).into(),
                    (3000.0 + 500.0 * r2).into(),
                    LinearModulator {
                        start_at: audio::ms(0),
                        end_at: audio::ms(80 + (75.0 * r3) as u32),
                    },
                );
            });
    }

    fn config_next_platform(platforms: &mut [Platform; 4], prev_index: usize, next_index: usize) {
        let next_is_above = get_random() < (u32::MAX >> 1); // 50%

        // TODO scale max gap with goat speed?
        let x = platforms[prev_index].pos.x
            + (platforms[prev_index].get_width() + 240 + get_random() as i32 % 120) as f32;
        let dy = 40.0 + (get_random() % 60) as f32;
        let y = platforms[prev_index].pos.y + if next_is_above { -dy } else { dy };
        //if y < 90.0 {
        //    y = 90.0;
        //} else if y > 150.0 {
        //    y = 150.0;
        //}
        let width: f32 = (240 + get_random() % 700) as f32; // max is 940
        platforms[next_index].set_width(width as i32);
        platforms[next_index].pos = Position::new(x, y);
    }

    pub fn reset(&mut self) {
        self.music_player.clear();
        self.audio.clear();
        audio::OUT.clear();
        audio::OUT.reset_all();
        let g = Game::new(false);
        *self = g;
    }
}
