use alloc::{boxed::Box, vec, vec::Vec};
use firefly_rust::{audio::*, get_random};

use crate::audio::Audio;
static GHOST_PITCHES: [f32; 7] = [220.0, 233.0, 311.0, 330.0, 349.0, 494.0, 622.0];

pub struct MusicPlayer {
    background: Node<Gain>,
    b: Node<Gain>,
    c: Node<Gain>,
    d: Node<Gain>,
    t: u32,

    notes: Vec<Box<dyn Playable>>,
    cur_note: usize,
}

static Q: u32 = 20;
static H1: u32 = 50;
static H2: u32 = H1 * 2;
static H3: u32 = H1 * 3;

macro_rules! notes {
    ($($array:expr);* $(;)?) => {
        vec![$(Box::new($array)),*]
    };
}

impl MusicPlayer {
    pub fn new(audio: &Audio) -> Self {
        Self {
            background: audio.music.add_gain(0.2),
            b: audio.music.add_gain(0.2),
            c: audio.music.add_gain(0.2),
            d: audio.music.add_gain(0.1),
            t: 0,

            //notes: vec![
            //    Box::new([Note::D5]),
            //    Box::new([Note::A5, Note::D3]),
            //    Box::new([Note::A4, Note::C4, Note::D3]),
            //    Box::new([Note::A5, Note::D3]),
            //    Box::new([Note::A3, Note::C3, Note::D2]),
            //    Box::new([Note::A4, Note::D2]),
            //],
            notes: notes!(
                [Note::C4(H2), Note::C3(H2)];
                [];
                [Note::G4(H1)];
                [];
                [Note::G4(H1), Note::C4(H1), Note::C3(H1)];
                [];
                [Note::G4(H1)];
                [];
                [Note::C4(H1), Note::C3(H1)];
                []; []; [];
                [Note::C4(H1), Note::C3(H1)];
                [];
                [Note::DS4(H1), Note::G3(H1), Note::G2(H1)];
                [];
                [Note::C4(H1), Note::C3(H1)];
                []; []; [];
                [Note::C4(H1), Note::C3(H1)];
                [];
                [Note::G3(H1), Note::F3(H1)];
                [];
                [Note::G3(H1), Note::F3(H2)];
                []; [];
                [Note::A3(H3), Note::C2(H2), Note::C3(H2)];
                []; []; [];
            ),
            cur_note: 0,
        }
    }

    pub fn update(&mut self) {
        if self.t == 0 {
            self.deep_drone(&self.background);
        } else if self.t % 60 * 20 == 0 {
            if Self::get_random_f32_01() < 0.3 {
                self.ghost(&self.b, 0.5 + Self::get_random_f32_01());
            }
        }

        let last_timing_t = Q * (self.notes.len() as u32 - 1);
        let note_t = self.t % (last_timing_t + 1);
        let timing = (note_t / Q) as usize;
        if timing >= self.cur_note {
            self.notes[self.cur_note].clear_then_play(&self.c);
            self.cur_note = (self.cur_note + 1) % self.notes.len();
        }

        self.t += 1;
    }

    fn deep_drone(&self, g: &Node<Gain>) {
        g.clear();
        g.modulate(0.05, 0.12, SineModulator { freq: 0.07.into() });
        g.add_sine(55.0.into(), 0.0);
        g.add_sine(58.3.into(), 0.0);
    }

    fn ghost(&self, g: &Node<Gain>, time_scale: f32) {
        g.clear();
        g.modulate(
            0.0,
            0.16,
            AdsrModulator {
                attack: ms((2000.0 * time_scale) as u32),
                decay: ms((4000.0 * time_scale) as u32),
                sustain: ms((7000.0 * time_scale) as u32),
                sustain_level: 0.4,
                release: ms((12000.0 * time_scale) as u32),
            },
        );

        let pitch: f32 = GHOST_PITCHES[get_random() as usize % 7];
        let t = g.add_triangle(pitch.into(), 0.0);
        t.modulate(
            (pitch - 7.0).into(),
            (pitch + 7.0).into(),
            SineModulator {
                freq: (0.05 + Self::get_random_f32_01() * 0.15).into(),
            },
        );
    }

    fn get_random_f32_01() -> f32 {
        get_random() as f32 / u32::MAX as f32
    }

    fn get_rnd(low: u32, high: u32) -> u32 {
        let r: u32 = get_random();
        low + (r % (high - low))
    }
}

trait Playable {
    fn play(&self, out: &Node<Gain>);
    fn clear_then_play(&self, out: &Node<Gain>);
}

#[derive(Clone, Copy)]
enum Note {
    C2(u32),
    CS2(u32),
    D2(u32),
    DS2(u32),
    E2(u32),
    F2(u32),
    FS2(u32),
    G2(u32),
    GS2(u32),
    A2(u32),
    AS2(u32),
    B2(u32),
    C3(u32),
    CS3(u32),
    D3(u32),
    DS3(u32),
    E3(u32),
    F3(u32),
    FS3(u32),
    G3(u32),
    GS3(u32),
    A3(u32),
    AS3(u32),
    B3(u32),
    C4(u32),
    CS4(u32),
    D4(u32),
    DS4(u32),
    E4(u32),
    F4(u32),
    FS4(u32),
    G4(u32),
    GS4(u32),
    A4(u32),
    AS4(u32),
    B4(u32),
    C5(u32),
    CS5(u32),
    D5(u32),
    DS5(u32),
    E5(u32),
    F5(u32),
    FS5(u32),
    G5(u32),
    GS5(u32),
    A5(u32),
    AS5(u32),
    B5(u32),
    C6(u32),
    CS6(u32),
    D6(u32),
    DS6(u32),
    E6(u32),
    F6(u32),
    FS6(u32),
    G6(u32),
    GS6(u32),
    A6(u32),
    AS6(u32),
    B6(u32),
    C7(u32),
    CS7(u32),
    D7(u32),
    DS7(u32),
    E7(u32),
    F7(u32),
    FS7(u32),
    G7(u32),
    GS7(u32),
    A7(u32),
    AS7(u32),
    B7(u32),
}

impl Note {
    fn freq(&self) -> f32 {
        match self {
            Note::C2(_) => 65.406,
            Note::CS2(_) => 69.296,
            Note::D2(_) => 73.416,
            Note::DS2(_) => 77.782,
            Note::E2(_) => 82.407,
            Note::F2(_) => 87.307,
            Note::FS2(_) => 92.499,
            Note::G2(_) => 97.999,
            Note::GS2(_) => 103.826,
            Note::A2(_) => 110.0,
            Note::AS2(_) => 116.541,
            Note::B2(_) => 123.471,
            Note::C3(_) => 130.813,
            Note::CS3(_) => 138.591,
            Note::D3(_) => 146.832,
            Note::DS3(_) => 155.563,
            Note::E3(_) => 164.814,
            Note::F3(_) => 174.614,
            Note::FS3(_) => 184.997,
            Note::G3(_) => 195.998,
            Note::GS3(_) => 207.652,
            Note::A3(_) => 220.0,
            Note::AS3(_) => 233.082,
            Note::B3(_) => 246.942,
            Note::C4(_) => 261.626,
            Note::CS4(_) => 277.183,
            Note::D4(_) => 293.665,
            Note::DS4(_) => 311.127,
            Note::E4(_) => 329.628,
            Note::F4(_) => 349.228,
            Note::FS4(_) => 369.994,
            Note::G4(_) => 391.995,
            Note::GS4(_) => 415.305,
            Note::A4(_) => 440.0,
            Note::AS4(_) => 466.164,
            Note::B4(_) => 493.883,
            Note::C5(_) => 523.251,
            Note::CS5(_) => 554.365,
            Note::D5(_) => 587.33,
            Note::DS5(_) => 622.254,
            Note::E5(_) => 659.255,
            Note::F5(_) => 698.456,
            Note::FS5(_) => 739.989,
            Note::G5(_) => 783.991,
            Note::GS5(_) => 830.609,
            Note::A5(_) => 880.0,
            Note::AS5(_) => 932.328,
            Note::B5(_) => 987.767,
            Note::C6(_) => 1046.502,
            Note::CS6(_) => 1108.731,
            Note::D6(_) => 1174.659,
            Note::DS6(_) => 1244.508,
            Note::E6(_) => 1318.51,
            Note::F6(_) => 1396.913,
            Note::FS6(_) => 1479.978,
            Note::G6(_) => 1567.982,
            Note::GS6(_) => 1661.219,
            Note::A6(_) => 1760.0,
            Note::AS6(_) => 1864.655,
            Note::B6(_) => 1975.533,
            Note::C7(_) => 2093.005,
            Note::CS7(_) => 2217.461,
            Note::D7(_) => 2349.318,
            Note::DS7(_) => 2489.016,
            Note::E7(_) => 2637.02,
            Note::F7(_) => 2793.826,
            Note::FS7(_) => 2959.955,
            Note::G7(_) => 3135.963,
            Note::GS7(_) => 3322.438,
            Note::A7(_) => 3520.0,
            Note::AS7(_) => 3729.31,
            Note::B7(_) => 3951.066,
        }
    }

    fn get_hold(self) -> u32 {
        match self {
            Note::C2(h) => h,
            Note::CS2(h) => h,
            Note::D2(h) => h,
            Note::DS2(h) => h,
            Note::E2(h) => h,
            Note::F2(h) => h,
            Note::FS2(h) => h,
            Note::G2(h) => h,
            Note::GS2(h) => h,
            Note::A2(h) => h,
            Note::AS2(h) => h,
            Note::B2(h) => h,
            Note::C3(h) => h,
            Note::CS3(h) => h,
            Note::D3(h) => h,
            Note::DS3(h) => h,
            Note::E3(h) => h,
            Note::F3(h) => h,
            Note::FS3(h) => h,
            Note::G3(h) => h,
            Note::GS3(h) => h,
            Note::A3(h) => h,
            Note::AS3(h) => h,
            Note::B3(h) => h,
            Note::C4(h) => h,
            Note::CS4(h) => h,
            Note::D4(h) => h,
            Note::DS4(h) => h,
            Note::E4(h) => h,
            Note::F4(h) => h,
            Note::FS4(h) => h,
            Note::G4(h) => h,
            Note::GS4(h) => h,
            Note::A4(h) => h,
            Note::AS4(h) => h,
            Note::B4(h) => h,
            Note::C5(h) => h,
            Note::CS5(h) => h,
            Note::D5(h) => h,
            Note::DS5(h) => h,
            Note::E5(h) => h,
            Note::F5(h) => h,
            Note::FS5(h) => h,
            Note::G5(h) => h,
            Note::GS5(h) => h,
            Note::A5(h) => h,
            Note::AS5(h) => h,
            Note::B5(h) => h,
            Note::C6(h) => h,
            Note::CS6(h) => h,
            Note::D6(h) => h,
            Note::DS6(h) => h,
            Note::E6(h) => h,
            Note::F6(h) => h,
            Note::FS6(h) => h,
            Note::G6(h) => h,
            Note::GS6(h) => h,
            Note::A6(h) => h,
            Note::AS6(h) => h,
            Note::B6(h) => h,
            Note::C7(h) => h,
            Note::CS7(h) => h,
            Note::D7(h) => h,
            Note::DS7(h) => h,
            Note::E7(h) => h,
            Note::F7(h) => h,
            Note::FS7(h) => h,
            Note::G7(h) => h,
            Note::GS7(h) => h,
            Note::A7(h) => h,
            Note::AS7(h) => h,
            Note::B7(h) => h,
        }
    }
}

impl Playable for Note {
    fn play(&self, out: &Node<Gain>) {
        out.set(0.33);
        let freq = self.freq();
        let subfreq = freq / 2.0;
        let supfreq = freq * 2.0;

        let rnd_variance = 0.95 - 0.1 * MusicPlayer::get_random_f32_01();

        out.modulate(
            0.0,
            1.0,
            AdsrModulator {
                attack: ms(30),
                decay: ms(100),
                sustain: ms(100 + self.get_hold()),
                sustain_level: 0.55,
                release: ms(100 + self.get_hold() + 70),
            },
        );

        let s = out.add_sine((freq * rnd_variance).into(), 0.0);
        let subs = out.add_sine((subfreq * rnd_variance).into(), 0.141414141414);
        let sups = out.add_triangle((supfreq * rnd_variance).into(), 0.737373737373);
    }

    fn clear_then_play(&self, out: &Node<Gain>) {
        out.clear();
        self.play(out);
    }
}

impl Playable for [Note; 0] {
    fn play(&self, out: &Node<Gain>) {
        let _ = out;
    }

    fn clear_then_play(&self, out: &Node<Gain>) {
        out.clear();
    }
}

impl Playable for [Note; 1] {
    fn play(&self, out: &Node<Gain>) {
        self[0].play(out);
    }

    fn clear_then_play(&self, out: &Node<Gain>) {
        self[0].play(out);
    }
}

impl Playable for [Note; 2] {
    fn play(&self, out: &Node<Gain>) {
        self[0].play(out);
        self[1].play(out);
    }

    fn clear_then_play(&self, out: &Node<Gain>) {
        self[0].clear_then_play(out);
        self[0].play(out);
        self[1].play(out);
    }
}

impl Playable for [Note; 3] {
    fn play(&self, out: &Node<Gain>) {
        self[0].play(out);
        self[1].play(out);
        self[2].play(out);
    }

    fn clear_then_play(&self, out: &Node<Gain>) {
        self[0].clear_then_play(out);
        self[0].play(out);
        self[1].play(out);
        self[2].play(out);
    }
}

impl Playable for [Note; 4] {
    fn play(&self, out: &Node<Gain>) {
        self[0].play(out);
        self[1].play(out);
        self[2].play(out);
        self[3].play(out);
    }

    fn clear_then_play(&self, out: &Node<Gain>) {
        self[0].clear_then_play(out);
        self[0].play(out);
        self[1].play(out);
        self[2].play(out);
        self[3].play(out);
    }
}
