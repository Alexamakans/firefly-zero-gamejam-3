use core::{
    f32::consts::PI,
    ops::{Index, IndexMut},
};

use firefly_rust::{Color, Style, draw_triangle, get_random, math::sin};

use crate::{
    game::{camera::Camera, position::Position},
    remap_color,
};

pub struct Background {
    layers: [Layer; 2],
}

impl Background {
    pub fn new() -> Self {
        Self {
            layers: [
                Layer::new(64.0, 110.0, 180.0, 1.0, Color::DarkGray, Color::Gray, 0.04),
                Layer::new(
                    48.0,
                    75.0,
                    105.0,
                    1.0,
                    Color::DarkBlue,
                    Color::LightGreen,
                    0.06,
                ),
            ],
        }
    }

    pub fn set_palette(&self) {
        remap_color!(
            Color::LightGreen => 163,194,186,
            Color::DarkBlue => 54,55,43,
        );
    }

    pub fn update(&mut self, camera: &Camera) {
        for layer in &mut self.layers {
            layer.update(camera);
        }
    }

    pub fn render(&self) {
        for layer in &self.layers {
            layer.render();
        }
    }
}

fn noise(x: f32) -> f32 {
    (1.0 + sin(2.0 * x) + sin(PI * x)) / 2.0
}

static BOTTOM_Y: f32 = 160.0;
static NUM_POINTS: usize = 16;
struct Layer {
    prev_camera_pos: Position,
    spacing: f32,
    min_height: f32,
    max_height: f32,
    roughness: f32,
    points: [Position; NUM_POINTS],

    bright: Style,
    dark: Style,
    scroll_x_factor: f32,
    height_offset: u32,

    start: usize,
}

impl Layer {
    fn new(
        spacing: f32,
        min_height: f32,
        max_height: f32,
        roughness: f32,
        dark: Color,
        bright: Color,
        scroll_x_factor: f32,
    ) -> Self {
        let mut points = [Position::new(0.0, 0.0); NUM_POINTS];
        let mut x = 0.0;
        let height_offset = get_random() % 1000;
        for p in &mut points {
            p.x = x;
            p.y = Self::y_from_x_annoying(x, min_height, max_height, height_offset, roughness);
            x += spacing;
        }
        Self {
            spacing,
            min_height,
            max_height,
            roughness,
            points,
            dark: Style {
                fill_color: dark,
                stroke_color: dark,
                stroke_width: 0,
            },
            bright: Style {
                fill_color: bright,
                stroke_color: bright,
                stroke_width: 0,
            },
            prev_camera_pos: Position::zero(),
            scroll_x_factor,
            height_offset,
            start: 0,
        }
    }

    fn update(&mut self, camera: &Camera) {
        let cd = camera.pos - self.prev_camera_pos;
        let dx = cd.x * self.scroll_x_factor;
        for p in &mut self.points {
            p.x -= dx;
        }
        if self[1].x < 0.0 {
            let x = self[self.points.len() - 1].x + self.spacing;
            self[0].x = x;
            self[0].y = self.y_from_x(x);
            self.shift();
        }
        self.prev_camera_pos = camera.pos;
    }

    fn render(&self) {
        for i in 0..self.points.len() - 1 {
            let a = self[i];
            let b = Position::new(a.x, BOTTOM_Y);
            let c = Position::new(a.x + self.spacing, BOTTOM_Y);
            let d = self[(i + 1) % self.points.len()];

            draw_triangle(a.into(), b.into(), c.into(), self.dark);
            draw_triangle(a.into(), d.into(), c.into(), self.bright);
        }
    }

    fn shift(&mut self) {
        self.start += 1;
        if self.start > self.points.len() {
            self.start = 0;
        }
    }

    fn y_from_x(&self, x: f32) -> f32 {
        Self::y_from_x_annoying(
            x,
            self.min_height,
            self.max_height,
            self.height_offset,
            self.roughness,
        )
    }

    fn y_from_x_annoying(x: f32, min: f32, max: f32, height_offset: u32, roughness: f32) -> f32 {
        BOTTOM_Y - min - noise(height_offset as f32 + x * roughness) * (max - min)
    }
}

impl Index<usize> for Layer {
    type Output = Position;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[(self.start + index) % self.points.len()]
    }
}

impl IndexMut<usize> for Layer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.points[(self.start + index) % self.points.len()]
    }
}
