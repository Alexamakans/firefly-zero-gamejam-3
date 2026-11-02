use alloc::vec;
use alloc::vec::Vec;
use firefly_rust::{
    Color, Image, ImageBuf, Point, Size, Style, SubImage, draw_circle, draw_sub_image, get_random,
    load_file_buf,
};

use crate::game::{camera::Camera, platform::Platform, position::Position};

/// not actually parallaxs probably
pub struct Parallax {
    layers: [Layer; 5],
    sheet: ImageBuf,
}

impl Parallax {
    pub fn new() -> Self {
        Self {
            layers: [
                Layer::new(40, vec![ItemKind::Bat]),
                Layer::new(65, vec![ItemKind::Bat, ItemKind::Pumpkin]),
                Layer::new(90, vec![ItemKind::Tree, ItemKind::Witch, ItemKind::Pumpkin]),
                Layer::new(60, vec![ItemKind::Witch, ItemKind::Pumpkin]),
                Layer::new(600, vec![ItemKind::Portal]),
            ],
            sheet: load_file_buf("background_props").expect("oh no").into(),
        }
    }

    pub fn update(&mut self, camera: &Camera, rightmost_platform: &Platform) {
        for layer in &mut self.layers {
            layer.update(camera, rightmost_platform);
        }
    }

    pub fn render(&self, camera: &Camera) {
        for layer in &self.layers {
            layer.render(&self.sheet, camera);
        }
    }

    pub fn on_collision(
        &mut self,
        pos: Position,
        r: f32,
        on_collision: &mut impl FnMut(&mut Item),
    ) {
        for layer in &mut self.layers {
            for item in &mut layer.items {
                if item.dead_ticks_elapsed > 0 {
                    continue;
                }
                let r2 = item.kind.get_radius() + r;
                let center = item.pos + Position::from(item.kind.get_size()) / 2.0;
                if pos.dist_sq(&center) < r2 * r2 {
                    on_collision(item);
                }
            }
        }
    }
}

struct Layer {
    items: Vec<Item>,
    ticks_per_spawn: u32,
    ticks_elapsed: u32,
    possible_kinds: Vec<ItemKind>,
}

impl Layer {
    fn new(ticks_per_spawn: u32, possible_kinds: Vec<ItemKind>) -> Self {
        Self {
            items: Vec::new(),
            ticks_per_spawn,
            possible_kinds,
            ticks_elapsed: get_random() % ticks_per_spawn,
        }
    }

    fn update(&mut self, camera: &Camera, rightmost_platform: &Platform) {
        self.items.retain(|item| item.pos.x - camera.pos.x > -50.0);
        for item in &mut self.items {
            item.t += 1;
            if item.t >= 3 {
                item.t = 0;
            }
            if item.dead_ticks_elapsed > 0 {
                item.dead_ticks_elapsed += 1;
            }
            match item.kind {
                ItemKind::Witch => {
                    item.pos.x += 0.5;
                }
                ItemKind::Bat => {
                    item.pos.x += (0.5 - ((get_random() % 100) as f32 / 100.0)) * 1.5;
                    item.pos.y += (0.5 - ((get_random() % 100) as f32 / 100.0)) * 1.5;
                }
                _ => (),
            }
        }
        if self.ticks_elapsed % self.ticks_per_spawn == 0 {
            self.ticks_elapsed = 0;
            self.spawn(camera, rightmost_platform);
        }
        self.ticks_elapsed += 1;
    }

    fn render(&self, sheet: &ImageBuf, camera: &Camera) {
        for item in &self.items {
            let p = item.pos - camera.pos;
            item.render(p, sheet);
        }
    }

    fn spawn(&mut self, camera: &Camera, platform: &Platform) {
        let mut pos = camera.pos + Position::new(300.0, (get_random() % 160) as f32);
        let index = get_random() as usize % self.possible_kinds.len();
        let kind = self.possible_kinds[index];
        match kind {
            ItemKind::Pumpkin | ItemKind::Tree | ItemKind::Portal => {
                let x = platform.pos.x
                    + (get_random() % (platform.get_width() as u32 - kind.get_width() as u32))
                        as f32;
                pos.x = x;
                pos.y = platform.pos.y - kind.get_height() as f32;
            }
            _ => (),
        }
        self.items.push(Item {
            kind,
            pos,
            dead_ticks_elapsed: 0,
            t: 0,
        });
    }
}

#[derive(Clone, Copy)]
pub enum ItemKind {
    Pumpkin,
    Witch,
    Bat,
    Tree,
    Portal,
}

impl<'a> ItemKind {
    fn get_sub_image(&self, img: &'a ImageBuf) -> SubImage<'a> {
        match self {
            ItemKind::Pumpkin => img.sub(Point::new(0, 0), self.get_size()),
            ItemKind::Witch => img.sub(Point::new(30, 11), self.get_size()),
            ItemKind::Bat => img.sub(Point::new(30, 28), self.get_size()),
            ItemKind::Tree => img.sub(Point::new(30, 0), self.get_size()),
            ItemKind::Portal => self.portal_thing(img, 0),
        }
    }

    fn portal_thing(&self, img: &'a ImageBuf, t: u32) -> SubImage<'a> {
        let a = t % 3;
        if a == 0 {
            img.sub(Point::new(0, 40), self.get_size())
        } else if a == 1 {
            img.sub(Point::new(31, 40), self.get_size())
        } else {
            img.sub(Point::new(62, 40), self.get_size())
        }
    }

    // dw about it
    fn get_width(&self) -> i32 {
        match self {
            ItemKind::Pumpkin => 30,
            ItemKind::Witch => 18,
            ItemKind::Bat => 7,
            ItemKind::Tree => 14,
            ItemKind::Portal => 31,
        }
    }

    // dw about it
    fn get_height(&self) -> i32 {
        match self {
            ItemKind::Pumpkin => 38,
            ItemKind::Witch => 17,
            ItemKind::Bat => 3,
            ItemKind::Tree => 11,
            ItemKind::Portal => 35,
        }
    }

    fn get_size(&self) -> Size {
        Size::new(self.get_width(), self.get_height())
    }

    fn get_radius(&self) -> f32 {
        let s = self.get_size();
        if let ItemKind::Portal = self {
            16.0
        } else {
            if s.width > s.height {
                s.width as f32 / 2.0
            } else {
                s.height as f32 / 2.0
            }
        }
    }
}

pub struct Item {
    pub kind: ItemKind,
    pub pos: Position,
    pub dead_ticks_elapsed: u32,
    t: u32,
}

impl Item {
    pub fn render(&self, p: Position, sheet: &ImageBuf) {
        if self.dead_ticks_elapsed > 0 {
            if self.dead_ticks_elapsed >= self.get_death_duration() {
                return;
            }
            let s = (self.get_death_duration() - self.dead_ticks_elapsed) as f32;
            let half_size: Position = (self.kind.get_size() / 2).into();
            let offset = Position::new(-s / 2.0, -s / 2.0);
            let mut style = Style {
                fill_color: Color::White,
                stroke_color: Color::None,
                stroke_width: 0,
            };
            draw_circle((p + offset / 2.0 + half_size).into(), s as i32, style);
            style.fill_color = Color::LightGray;
            draw_circle(
                (p + offset * 0.75 / 2.0 + half_size).into(),
                (s * 0.75) as i32,
                style,
            );
            style.fill_color = Color::Gray;
            draw_circle(
                (p + offset * 0.33 / 2.0 + half_size).into(),
                (s * 0.33) as i32,
                style,
            );
            style.fill_color = Color::DarkGray;
            draw_circle(
                (p + offset * 0.25 / 2.0 + half_size).into(),
                (s * 0.25) as i32,
                style,
            );
        } else {
            if let ItemKind::Portal = self.kind {
                draw_sub_image(&self.kind.portal_thing(sheet, self.t), p.into());
            } else {
                draw_sub_image(&self.kind.get_sub_image(sheet), p.into());
            }
        }
    }

    fn get_death_duration(&self) -> u32 {
        match self.kind {
            ItemKind::Pumpkin => 16,
            ItemKind::Witch => 16,
            ItemKind::Bat => 8,
            ItemKind::Tree => 16,
            ItemKind::Portal => 32,
        }
    }
}

impl Clone for Item {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            pos: self.pos.clone(),
            dead_ticks_elapsed: 0,
            t: 0,
        }
    }
}
