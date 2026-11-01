use firefly_rust::{
    Size,
    audio::{
        self, AdsrModulator, HoldModulator, LinearModulator, Sawtooth, SawtoothModulator,
        SineModulator,
    },
    get_random, load_file_buf,
    math::{abs, sqrt},
};

use crate::{
    audio::Audio,
    game::{
        animation::{Animation, Spritesheet},
        camera::Camera,
        platform::Platform,
        position::Position,
    },
    input::Input,
};

pub struct Goat {
    pub pos: Position,
    pub bounds: Position,
    pub vel: Position,
    anim: Animation,
    jump_spritesheet: Spritesheet,
    dash_spritesheet: Spritesheet,

    jump: JumpController,
    dash: DashController,

    bounce_frame_offset: Position,

    pub collided_platform: Platform,
    pub collides_platform: bool,
    pub collides_y: f32,

    pub dead: bool,

    elapsed_ticks: u32,
}

impl Goat {
    pub fn new(pos: Position) -> Self {
        Self {
            pos,
            bounds: Position::new(25.0, 15.0),
            vel: Position::zero(),
            anim: Animation::new("goat", Size::new(25, 15), 15, 2),
            jump_spritesheet: Spritesheet::new(
                load_file_buf("goat_jump")
                    .expect("failed loading goat jump spritesheet")
                    .into(),
                Size::new(25, 15),
            ),
            dash_spritesheet: Spritesheet::new(
                load_file_buf("goat_dash")
                    .expect("failed loading goat jump spritesheet")
                    .into(),
                Size::new(25, 15),
            ),
            bounce_frame_offset: Position::new(0.0, -6.0),

            jump: JumpController::new(2),
            dash: DashController::new(2),

            // TODO: jump buffer probably
            collided_platform: Platform::new(Position::zero(), 0),
            collides_platform: false,
            collides_y: 0.0,

            dead: false,

            elapsed_ticks: 0,
        }
    }

    pub fn update(&mut self, input: &Input, audio: &mut Audio) {
        self.elapsed_ticks += 1;

        let mut vel = self.vel;
        self.dash.update(input, audio, &mut vel);
        self.vel = vel;

        if !self.dash.is_dashing() {
            if self.dash.was_dashing {
                self.dash.was_dashing = false;
            }
            self.vel.x = 2.35 + sqrt(self.pos.x) * 0.025;
        }

        let gravity = if input.held.s { 0.025 } else { 0.08 };
        let non_linearity_or_something = 0.0011 * abs(self.vel.y);
        self.vel.y += gravity + non_linearity_or_something;

        if self.should_die() {
            self.kill(audio);
            return;
        }

        let mut pos = self.pos;
        let mut vel = self.vel;
        let mut jump = self.jump;
        jump.update(self, &mut pos, &mut vel, input, audio);
        self.pos = pos;
        self.vel = vel;
        self.jump = jump;

        self.pos += self.vel;
        self.anim.update();

        if self.just_grounded() {
            self.play_land_sound(audio);
        }

        if self.is_grounded() {
            self.dash.reset();
            self.jump.was_grounded = true;
            self.play_run_sound(audio);
        } else {
            self.jump.was_grounded = false;
        }
    }

    pub fn render(&self, camera: &Camera) {
        let p = self.pos - camera.pos;
        if self.is_grounded() {
            if self.anim.get_frame() == 1 {
                self.anim.render(p + self.bounce_frame_offset);
            } else {
                self.anim.render(p);
            }
        } else if self.dash.is_dashing() {
            self.dash_spritesheet
                .render(self.dash.ticks_elapsed as i32, p);
        } else {
            if abs(self.vel.y) < 0.1 {
                self.jump_spritesheet.render(1, p);
            } else if self.vel.y < 0.0 {
                self.jump_spritesheet.render(0, p);
            } else {
                self.jump_spritesheet.render(2, p);
            }
        }
    }

    fn is_grounded(&self) -> bool {
        self.collides_platform
    }

    fn just_grounded(&self) -> bool {
        !self.jump.was_grounded && self.is_grounded()
    }

    fn play_run_sound(&self, audio: &mut Audio) {
        if self.anim.get_frame() == 1 && self.anim.just_changed_frame() {
            boing(audio, 0.3, 0.0, 0.75);
        }
    }

    fn play_land_sound(&self, audio: &mut Audio) {
        audio.play_player_sfx(|parent: &audio::Node<audio::Gain>| {
            let g = parent.add_gain(0.0);
            g.modulate(
                0.15,
                0.0,
                LinearModulator {
                    start_at: audio::ms(0),
                    end_at: audio::ms(80),
                },
            );

            //let lp = g.add_low_pass(138.0.into(), q);
            g.add_noise(get_random() as i32);

            let a = g.add_sawtooth((140.0 + 10.0).into(), 0.0);
            a.modulate(
                150.0.into(),
                120.0.into(),
                SawtoothModulator {
                    period: audio::ms(10),
                },
            );
            g.add_sawtooth((140.0 - 10.0).into(), 0.4);
        });
    }

    fn should_die(&self) -> bool {
        if self.collides_platform {
            if self.collided_platform.pos.y < self.pos.y + self.bounds.y - 7.0 {
                return true;
            }
        }
        return false;
    }

    pub fn kill(&mut self, audio: &mut Audio) {
        if self.dead {
            return;
        }
        self.dead = true;
        self.play_death_sound(audio);
    }

    fn play_death_sound(&self, audio: &mut Audio) {
        audio.play_player_sfx(|parent: &audio::Node<audio::Gain>| {
            let g = parent.add_gain(0.0);
            g.modulate(
                0.0,
                1.0,
                AdsrModulator {
                    attack: audio::ms(1),
                    decay: audio::ms(150),
                    sustain: audio::ms(250),
                    sustain_level: 0.5,
                    release: audio::ms(1000),
                },
            );
            let g2 = g.add_gain(0.5);
            g2.modulate(
                0.0,
                1.0,
                SawtoothModulator {
                    period: audio::ms(10),
                },
            );
            g.add_sawtooth(40.0.into(), 0.0);
            g.add_sawtooth(60.0.into(), 0.7171);
        });
    }

    pub fn get_hud_data(&self) -> HUDData {
        HUDData {
            jumps: self.jump.extra_charges,
            jumps_max: self.jump.extra_charges_max,
            dashes: self.dash.charges,
            dashes_max: self.dash.charges_max,
        }
    }
}

#[derive(Debug)]
pub struct HUDData {
    pub jumps: u32,
    pub jumps_max: u32,
    pub dashes: u32,
    pub dashes_max: u32,
}

#[derive(Clone, Copy)]
struct JumpController {
    t: u32,
    jumping: bool,
    released: bool,
    tiny_factor: f32,
    coyote_ticks: u32,
    coyote_elapsed_ticks: u32,
    extra_charges_max: u32,
    extra_charges: u32,
    was_grounded: bool,
}

impl JumpController {
    fn new(charges: u32) -> Self {
        Self {
            t: 0,
            jumping: false,
            released: false,
            tiny_factor: 0.1,
            coyote_ticks: 7,
            coyote_elapsed_ticks: 0,
            extra_charges_max: charges,
            extra_charges: charges,
            was_grounded: false,
        }
    }

    fn can_jump(&self, goat: &Goat) -> bool {
        goat.is_grounded()
            || self.coyote_elapsed_ticks <= self.coyote_ticks
            || self.extra_charges > 0
    }

    fn requires_extra_jump_charges(&self, goat: &Goat) -> bool {
        !goat.is_grounded() && self.coyote_elapsed_ticks > self.coyote_ticks
    }

    fn update(
        &mut self,
        goat: &Goat,
        goat_pos: &mut Position,
        goat_vel: &mut Position,
        input: &Input,
        audio: &mut Audio,
    ) {
        if goat.is_grounded() {
            self.coyote_elapsed_ticks = 0;
            self.jumping = false;
            self.t = 0;
            goat_pos.y = goat.collides_y;
            goat_vel.y = 0.0;
            self.extra_charges = self.extra_charges_max;
        } else {
            self.coyote_elapsed_ticks += 1;
        }

        if input.released.s {
            self.released = true;
        }
        if self.jumping {
            let falloff_t = 90;
            if input.held.s || self.t <= (falloff_t as f32 * self.tiny_factor) as u32 {
                let vy = Self::power(self.t, falloff_t, falloff_t + 10);
                if vy != 0.0 {
                    goat_vel.y = -vy;
                    self.t += 1;
                }
            } else if self.released && self.t > (falloff_t as f32 * self.tiny_factor) as u32 {
                self.t = 0;
                self.jumping = false;
                if goat_vel.y < 0.0 {
                    goat_vel.y = 0.0;
                }
            }
        } else if input.pressed.s && self.can_jump(goat) {
            if self.requires_extra_jump_charges(goat) {
                self.extra_charges -= 1;
            }
            self.play_sound(audio);
            self.jumping = true;
            self.released = false;
            let vy = Self::power(self.t, 30, 60);
            if vy != 0.0 {
                goat_vel.y = -vy;
                self.t += 1;
            }
        }
    }

    fn play_sound(&self, audio: &mut Audio) {
        boing(audio, 0.4, 100.0, 0.45);
    }

    pub fn power(t: u32, falloff_t: u32, dur_t: u32) -> f32 {
        if t > dur_t {
            return 0.0;
        }
        assert!(falloff_t <= dur_t);
        let scale = 4.8;
        let jump_power = 1.0 * scale;
        let t_f = t as f32 / scale;
        let falloff_t_f = falloff_t as f32 / scale;
        let dur_t_f = dur_t as f32 / scale;
        if t_f <= falloff_t_f {
            let perc = 1.0 - t_f / (falloff_t_f - t_f);
            jump_power * perc * perc
        } else if t_f < dur_t_f {
            let perc = 1.0 - (t_f - falloff_t_f) / (dur_t_f - falloff_t_f);
            jump_power * perc
        } else {
            0.0
        }
    }
}

fn boing(audio: &mut Audio, gain: f32, freq_offset: f32, duration_scale: f32) {
    audio.play_player_sfx(|parent: &audio::Node<audio::Gain>| {
        let g = parent.add_gain(0.0);
        g.modulate(
            0.0,
            gain,
            AdsrModulator {
                attack: audio::ms((20.0 * duration_scale) as u32),
                decay: audio::ms((80.0 * duration_scale) as u32),
                sustain_level: 0.5,
                sustain: audio::ms((100.0 * duration_scale) as u32),
                release: audio::ms((200.0 * duration_scale) as u32),
            },
        );
        //g.modulate(
        //    0.5,
        //    0.0,
        //    HoldModulator {
        //        time: audio::ms(200),
        //    },
        //);
        let start_freq = 190.0 + (get_random() % 20) as f32 + freq_offset;
        let s = g.add_triangle(start_freq.into(), 0.0);
        let end_freq = start_freq as f32 * (1.6 + ((get_random() as f32 % 100.0) / 100.0) * 0.5);
        s.modulate(
            start_freq.into(),
            end_freq.into(),
            LinearModulator {
                start_at: audio::ms(0),
                end_at: audio::ms((200.0 * duration_scale) as u32),
            },
        );
    });
}

struct DashController {
    ticks: u32,
    ticks_elapsed: u32,
    was_dashing: bool,
    charges_max: u32,
    charges: u32,
}

impl DashController {
    fn new(charges: u32) -> Self {
        Self {
            ticks: 16,
            ticks_elapsed: 100, // high to make no dash start blabla
            was_dashing: false,
            charges_max: charges,
            charges,
        }
    }

    fn update(&mut self, input: &Input, audio: &mut Audio, vel: &mut Position) {
        self.ticks_elapsed += 1;

        if input.pressed.e && self.ticks_elapsed > self.ticks && self.charges > 0 {
            self.charges -= 1;
            self.ticks_elapsed = 0;
            self.was_dashing = true;
            self.play_sound(audio);
        }
        if self.is_dashing() {
            vel.x = self.dash_vel(self.ticks_elapsed);
        }
    }

    fn play_sound(&self, audio: &mut Audio) {
        audio.play_player_sfx(|parent: &audio::Node<audio::Gain>| {
            let g = parent.add_gain(0.0);
            g.modulate(
                0.0,
                0.4,
                SineModulator {
                    freq: ((16.0 / 60.0) / 4.0).into(),
                },
            );
            let g2 = g.add_gain(0.0);
            g2.modulate(
                1.0,
                0.0,
                HoldModulator {
                    time: audio::ms(((16.0 / 60.0) * 1000.0) as u32),
                },
            );
            let ms_per_tick = audio::ms((1000.0 / 60.0) as u32);

            let s = g2.add_sine(0.0.into(), 0.0);
            s.modulate(
                80.0.into(),
                2000.0.into(),
                AdsrModulator {
                    attack: ms_per_tick * 3,
                    decay: ms_per_tick * 6,
                    sustain: ms_per_tick * 9,
                    sustain_level: 0.3,
                    release: ms_per_tick * 16,
                },
            );
            let t = g2.add_sawtooth(0.0.into(), 0.5);
            t.modulate(
                80.0.into(),
                2000.0.into(),
                AdsrModulator {
                    attack: ms_per_tick * 5,
                    decay: ms_per_tick * 8,
                    sustain: ms_per_tick * 11,
                    sustain_level: 0.3,
                    release: ms_per_tick * 16,
                },
            );
        });
    }

    fn dash_vel(&self, t: u32) -> f32 {
        assert!(self.ticks % 2 == 0);
        let p = 3.5;
        if t < 6 {
            0.0
        } else if t < self.ticks / 2 {
            t as f32 * p
        } else {
            ((self.ticks as f32 / 2.0) * p) - ((t - (self.ticks / 2)) as f32 * p)
        }
    }

    fn is_dashing(&self) -> bool {
        self.ticks_elapsed < self.ticks
    }

    fn reset(&mut self) {
        self.charges = self.charges_max;
    }
}
