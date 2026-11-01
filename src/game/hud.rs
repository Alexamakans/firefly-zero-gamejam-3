use alloc::format;
use firefly_rust::{
    Color, FontBuf, Point, Style, draw_circle, draw_text, load_file_buf, log_debug,
};

use crate::game::goat::Goat;

pub struct HUD {
    font: FontBuf,
}

static AVAILABLE_STYLE: Style = Style {
    fill_color: Color::Green,
    stroke_color: Color::Black,
    stroke_width: 1,
};

static USED_STYLE: Style = Style {
    fill_color: Color::Red,
    stroke_color: Color::Black,
    stroke_width: 1,
};

impl HUD {
    pub fn new() -> Self {
        Self {
            font: load_file_buf("default_font")
                .expect("failed loading default font")
                .into(),
        }
    }

    pub fn render(&self, score: u32, goat: &Goat) {
        draw_text(
            &format!("score: {}", score),
            &self.font,
            Point::new(2, 12),
            Color::Red,
        );

        let data = goat.get_hud_data();
        log_debug!("datttt: {:#?}", data);
        {
            let base_p = Point::new(0, 20);
            for i in 0..data.jumps_max {
                let used = i >= data.jumps;
                let style = if used { USED_STYLE } else { AVAILABLE_STYLE };

                draw_circle(base_p + Point::new(i as i32 * 13, 0), 12, style);
            }
        }
        {
            let base_p = Point::new(0, 33);
            for i in 0..data.dashes_max {
                let used = i >= data.dashes;
                let style = if used { USED_STYLE } else { AVAILABLE_STYLE };

                draw_circle(base_p + Point::new(i as i32 * 13, 0), 12, style);
            }
        }
    }
}
