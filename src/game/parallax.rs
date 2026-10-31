use alloc::vec;
use alloc::vec::Vec;
use firefly_rust::{
    Image, ImageBuf, Point, Size, SubImage, draw_sub_image, get_random, load_file_buf,
};

use crate::game::{camera::Camera, platform::Platform, position::Position};

/// not actually parallaxs probably
pub struct Parallax {
    layers: [Layer; 4],
    sheet: ImageBuf,
}

impl Parallax {
    pub fn new() -> Self {
        Self {
            layers: [
                Layer::new(1.2, 20, vec![ItemKind::Bat]),
                Layer::new(0.8, 45, vec![ItemKind::Bat, ItemKind::Pumpkin]),
                Layer::new(
                    0.6,
                    60,
                    vec![ItemKind::Tree, ItemKind::Witch, ItemKind::Pumpkin],
                ),
                Layer::new(0.2, 40, vec![ItemKind::Witch, ItemKind::Pumpkin]),
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
}

struct Layer {
    items: Vec<Item>,
    factor: f32,
    ticks_per_spawn: u32,
    ticks_elapsed: u32,
    possible_kinds: Vec<ItemKind>,
}

impl Layer {
    fn new(factor: f32, ticks_per_spawn: u32, possible_kinds: Vec<ItemKind>) -> Self {
        Self {
            items: Vec::new(),
            factor,
            ticks_per_spawn,
            possible_kinds,
            ticks_elapsed: get_random() % ticks_per_spawn,
        }
    }

    fn update(&mut self, camera: &Camera, rightmost_platform: &Platform) {
        self.items.retain(|item| item.pos.x - camera.pos.x > -50.0);
        for item in &mut self.items {
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
            draw_sub_image(&item.kind.get_sub_image(sheet), p.into());
        }
    }

    fn spawn(&mut self, camera: &Camera, platform: &Platform) {
        let mut pos = camera.pos + Position::new(300.0, (get_random() % 160) as f32);
        let index = get_random() as usize % self.possible_kinds.len();
        let kind = self.possible_kinds[index];
        match kind {
            ItemKind::Pumpkin | ItemKind::Tree => {
                if pos.x < platform.pos.x {
                    pos.x = platform.pos.x;
                }
                if pos.x > platform.pos.x + (platform.get_width() - kind.get_width()) as f32 {
                    pos.x = platform.pos.x + (platform.get_width() - kind.get_width()) as f32;
                }
                pos.y = platform.pos.y - kind.get_height() as f32;
            }
            _ => (),
        }
        self.items.push(Item { kind, pos });
    }
}

#[derive(Clone, Copy)]
enum ItemKind {
    Pumpkin,
    Witch,
    Bat,
    Tree,
}

impl<'a> ItemKind {
    fn get_sub_image(&self, img: &'a ImageBuf) -> SubImage<'a> {
        match self {
            ItemKind::Pumpkin => img.sub(Point::new(0, 0), Size::new(30, 38)),
            ItemKind::Witch => img.sub(Point::new(30, 11), Size::new(18, 17)),
            ItemKind::Bat => img.sub(Point::new(30, 28), Size::new(7, 3)),
            ItemKind::Tree => img.sub(Point::new(30, 0), Size::new(14, 11)),
        }
    }

    // dw about it
    fn get_width(&self) -> i32 {
        match self {
            ItemKind::Pumpkin => 30,
            ItemKind::Witch => 18,
            ItemKind::Bat => 7,
            ItemKind::Tree => 14,
        }
    }

    // dw about it
    fn get_height(&self) -> i32 {
        match self {
            ItemKind::Pumpkin => 38,
            ItemKind::Witch => 17,
            ItemKind::Bat => 3,
            ItemKind::Tree => 11,
        }
    }
}

struct Item {
    kind: ItemKind,
    pos: Position,
}

impl Clone for Item {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            pos: self.pos.clone(),
        }
    }
}
