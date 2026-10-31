use firefly_rust::audio;

pub fn swoowowowo(parent: &audio::Node<audio::Gain>) {
    let gain_1 = parent.add_gain(0.0);
    gain_1.modulate(
        0.0,
        0.05,
        audio::AdsrModulator {
            attack: audio::samples(20600),
            decay: audio::samples(34700),
            sustain: audio::samples(58500),
            sustain_level: 0.29,
            release: audio::samples(146700),
        },
    );
    let square_2 = gain_1.add_square(185.0.into(), 0.86);
    square_2.modulate(
        185.0.into(),
        281.0.into(),
        audio::SineModulator { freq: 3.811.into() },
    );
    let sine_3 = gain_1.add_sine(84.0.into(), 0.19);
    sine_3.modulate(
        36.258.into(),
        512.333.into(),
        audio::SquareModulator {
            period: audio::samples((audio::SAMPLE_RATE as f32 / 1.91) as u32),
        },
    );
}
