use firefly_rust::{Color, Size, Style, draw_rect, draw_triangle};

use crate::game::camera::Camera;
use crate::game::position::Position;

#[derive(Copy, Clone)]
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
    fill_color: Color::Cyan,
    stroke_color: Color::Red,
    stroke_width: 1,
};

impl Platform {
    pub fn new(pos: Position, width: i32) -> Self {
        Self {
            pos,
            size: Size::new(width, 100),
            style: STYLE,
        }
    }

    pub fn render(&self, camera: &Camera) {
        let p = self.pos - camera.pos;
        draw_rect(p.into(), self.size, self.style);
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

    pub fn render_indicator(&self, camera: &Camera) {
        let p = self.pos - camera.pos;
        let min_x = 20.0;
        let min_y = 20.0;
        let max_x = 220.0;
        let max_y = 140.0;
        let clamped_p = {
            let mut v = p.clone();
            v.clamp_to(min_x, min_y, max_x, max_y);
            v
        };
        let d = p.dist_sq(&clamped_p) * 0.00001;
        if p.x + self.size.width as f32 / 4.0 > 0.0 && d > 0.0 {
            let s = if d < 1.5 { 2.0 - d } else { 0.5 };
            let w = s * 5.0;
            let h = w;
            let mut a: Option<Position> = None;
            let mut b: Option<Position> = None;
            let mut c: Option<Position> = None;
            let mut rp: Option<Position> = None;
            let mut pw = (self.size.width as f32 / 20.0) * s;
            if pw < 5.0 {
                pw = 5.0;
            }
            let mut ph = (self.size.height as f32 / 20.0) * s;
            if ph < 3.0 {
                ph = 3.0;
            }
            if p.x > max_x {
                let mut a1 = Position::new(clamped_p.x, clamped_p.y); // + half_platform_height;
                a1.clamp_to(min_x, min_y, max_x, max_y);
                a = Some(a1);
                b = Some(a.unwrap() + Position::new(-w, h / 2.0));
                c = Some(a.unwrap() + Position::new(-w, -h / 2.0));

                let offset = Position::new(-pw, -ph / 2.0);
                let right = b.unwrap() - Position::new(w * 1.3, h / 2.0);
                rp = Some(right + offset);
            } else if p.y < min_y {
                let mut a1 = Position::new(120.0, clamped_p.y);
                a1.clamp_to(p.x, min_y, p.x + self.size.width as f32, max_y);
                a = Some(a1);
                b = Some(a.unwrap() + Position::new(w / 2.0, h));
                c = Some(a.unwrap() + Position::new(-w / 2.0, h));

                let offset = Position::new(-pw / 2.0, 0.0);
                let up = b.unwrap() - Position::new(w / 2.0, -h * 1.3);
                rp = Some(up + offset);
            } else if p.y > max_y {
                let mut a1 = Position::new(120.0, clamped_p.y);
                a1.clamp_to(p.x, min_y, p.x + self.size.width as f32, max_y);
                a = Some(a1);
                b = Some(a.unwrap() + Position::new(w / 2.0, -h));
                c = Some(a.unwrap() + Position::new(-w / 2.0, -h));

                let offset = Position::new(-pw / 2.0, 0.0);
                let up = b.unwrap() - Position::new(w / 2.0, h * 1.3);
                rp = Some(up + offset);
            }

            if let Some(a) = a
                && let Some(b) = b
                && let Some(c) = c
                && let Some(rp) = rp
            {
                draw_triangle(a.into(), b.into(), c.into(), TRIANGLE_STYLE);
                let mut style = self.style;
                style.stroke_width = 1;
                style.fill_color = Color::LightGray;
                draw_rect(rp.into(), Size::new(pw as i32, ph as i32), style);
            }
        }
    }
}
