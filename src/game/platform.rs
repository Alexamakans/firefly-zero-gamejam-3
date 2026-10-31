use firefly_rust::{Color, Size, Style, draw_rect, draw_triangle, log_debug};

use crate::game::camera::Camera;
use crate::game::position::Position;

#[derive(Debug)]
pub struct Platform {
    pub pos: Position,
    size: Size,
    style: Style,
}

static STYLE: Style = Style {
    fill_color: Color::DarkGray,
    stroke_color: Color::Black,
    stroke_width: 1,
};

static TRIANGLE_STYLE: Style = Style {
    fill_color: Color::LightGray,
    stroke_color: Color::None,
    stroke_width: 0,
};

impl Platform {
    pub fn new(pos: Position, width: i32) -> Self {
        Self {
            pos,
            size: Size::new(width, 10),
            style: STYLE,
        }
    }

    pub fn render(&self, camera: &Camera) {
        let p = self.pos - camera.pos;
        let d = 1.0 - (p.x - 240.0) / 300.0;
        log_debug!("d: {:#?}", d);
        if p.x > 235.0 && d > 0.0 {
            let h = self.size.height as f32 * d;
            let w = 10.0 * d;
            let y = if p.y < h / 2.0 {
                0.0
            } else if p.y > 160.0 + h / 2.0 {
                160.0 - h
            } else {
                p.y
            };
            let cx = 225.0 + w / 2.0;
            let a = Position::new(cx - w / 2.0, y);
            let b = a + Position::new(w, h / 2.0);
            let c = a + Position::new(0.0, h);
            draw_triangle(a.into(), b.into(), c.into(), TRIANGLE_STYLE);
        }
        draw_rect((self.pos - camera.pos).into(), self.size, self.style);
    }

    pub fn set_width(&mut self, width: i32) {
        self.size.width = width;
    }

    pub fn get_width(&self) -> i32 {
        self.size.width
    }

    pub fn collision(&self, p: &Position) -> bool {
        if p.x < self.pos.x {
            return false;
        }
        if p.x > self.pos.x + self.size.width as f32 {
            return false;
        }
        if p.y < self.pos.y {
            return false;
        }
        if p.y > self.pos.y + self.size.height as f32 {
            return false;
        }
        return true;
    }
}
