mod animation;
mod background;
mod camera;
mod goat;
mod macros;
mod parallax;
mod platform;
mod position;

use crate::audio::Audio;
use crate::game::background::Background;
use crate::game::camera::Camera;
use crate::game::parallax::Parallax;
use crate::game::platform::Platform;
use crate::game::position::Position;
use crate::input::Input;
use crate::remap_color;
use alloc::format;
use firefly_rust::audio::{Gain, HoldModulator, Node, ms};
use firefly_rust::{
    Color, FontBuf, Peer, Point, clear_screen, draw_text, get_random, load_file_buf, log_debug,
    read_buttons, read_pad,
};
use goat::Goat;

pub struct Game {
    audio: Audio,
    input: Input,

    camera: Camera,
    camera_goat_offset: Position,

    goat: Goat,
    platforms: [Platform; 4],
    /// not really the next_platform_index, dw aobut it
    next_platform_index: usize,
    leftmost_platform_index: usize,

    parallax: Parallax,
    background: Background,

    score: u32,
    font: FontBuf,
}

fn mysound(parent: &Node<Gain>) {
    let s = parent.add_gain(0.5);
    s.modulate(1.0, 0.0, HoldModulator { time: ms(1000) });
    //s.add_sine((440.0 + 20.0).into(), 0.0);
    //s.add_sine((440.0 - 20.0).into(), 0.0);
    s.add_sawtooth((440.0 + 20.0).into(), 0.0);
    s.add_sawtooth((440.0 - 20.0).into(), 0.0);
}

impl Game {
    pub fn new() -> Self {
        remap_color!(Color::Blue => 46, 48, 64);

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

        let camera = Camera::new(Position::zero());
        Self {
            audio: Audio::new(),
            input: Input {
                old,
                pressed: old,
                held: old,
                released: old,
                pad: read_pad(Peer::COMBINED),
            },

            camera,
            camera_goat_offset: Position::new(-5.0, -90.0),

            goat: Goat::new(goat_pos),
            platforms,
            next_platform_index: 0,
            leftmost_platform_index: 0,

            parallax: Parallax::new(),
            background: Background::new(),

            score: 0,

            font: load_file_buf("default_font")
                .expect("failed loading default font")
                .into(),
        }
    }

    pub fn update(&mut self) {
        log_debug!("------------------------------------");
        self.input.update(Peer::COMBINED);

        for i in 0..self.platforms.len() {
            let x = self.platforms[i].pos.x - self.camera.pos.x;
            if (x + self.platforms[i].get_width() as f32) < 0.0 {
                let prev_index = if i == 0 {
                    self.platforms.len() - 1
                } else {
                    i - 1
                };
                Self::config_next_platform(&mut self.platforms, prev_index, i);
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

        self.camera.pos.lerp(
            0.08,
            self.goat.pos + self.camera_goat_offset
                - self.camera_goat_offset * self.goat.vel * 0.18,
        );

        self.background.update(&self.camera);

        if self.goat.pos.x > old_goat_pos.x {
            self.score += (self.goat.pos.x - old_goat_pos.x) as u32;
        }
    }

    pub fn render(&self) {
        clear_screen(Color::Blue);

        self.background.render();

        self.parallax.render(&self.camera);
        for platform in &self.platforms {
            platform.render(&self.camera);
        }
        self.goat.render(&self.camera);

        draw_text(
            &format!("score: {}", self.score),
            &self.font,
            Point::new(2, 12),
            Color::Red,
        );
    }

    fn generate_platform(&mut self) {
        let prev_index = self.next_platform_index;
        self.next_platform_index = (self.next_platform_index + 1) % self.platforms.len();

        let prev_platform_pos = self.platforms[prev_index].pos;
        let p = &mut self.platforms[self.next_platform_index];

        let width: f32 = (80 + get_random() % 20) as f32;
        let mut y = prev_platform_pos.y - (40 + get_random() % 80) as f32;
        if y < 90.0 {
            y = 90.0;
        } else if y > 150.0 {
            y = 150.0;
        }
        p.set_width(width as i32);
        p.pos = Position::new(160.0, y);
    }

    fn config_next_platform(platforms: &mut [Platform; 4], prev_index: usize, next_index: usize) {
        let next_is_above = get_random() < (u32::MAX >> 1);

        // TODO scale max gap with goat speed?
        let x = platforms[prev_index].pos.x
            + (platforms[prev_index].get_width() + 120 + get_random() as i32 % 80) as f32;
        let dy = 40.0 + (get_random() % 60) as f32;
        let y = platforms[prev_index].pos.y + if next_is_above { -dy } else { dy };
        //if y < 90.0 {
        //    y = 90.0;
        //} else if y > 150.0 {
        //    y = 150.0;
        //}
        let width: f32 = (120 + get_random() % 120) as f32; // max is 240
        platforms[next_index].set_width(width as i32);
        platforms[next_index].pos = Position::new(x, y);
    }
}
