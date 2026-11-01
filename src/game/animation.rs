use firefly_rust::{Image, ImageBuf, Point, Size, SubImage, draw_sub_image, load_file_buf};

pub struct Spritesheet {
    pub img: ImageBuf,
    cell_size: Size,
}

impl<'a> Spritesheet {
    fn cell(&'a self, col: i32, row: i32) -> SubImage<'a> {
        self.img.sub(
            Point::new(col * self.cell_size.width, row * self.cell_size.height),
            Size::new(self.cell_size.width, self.cell_size.height),
        )
    }

    pub fn render(&self, frame: i32, p: impl Into<Point>) {
        draw_sub_image(&self.cell(frame, 0), p.into());
    }
}

impl Spritesheet {
    pub fn new(img: ImageBuf, cell_size: Size) -> Self {
        Self { img, cell_size }
    }
}

pub struct Animation {
    pub spritesheet: Spritesheet,
    ticks_per_frame: u32,
    cur_frame_elapsed: u32,
    cur_frame: i32,
    num_frames: i32,
    changed_frame: bool,
}

impl Animation {
    pub fn new(path: &str, cell_size: Size, ticks_per_frame: u32, num_frames: i32) -> Self {
        Self {
            spritesheet: Spritesheet::new(
                load_file_buf(path).expect("img failed loading").into(),
                cell_size,
            ),
            ticks_per_frame,
            cur_frame_elapsed: 0,
            cur_frame: 0,
            num_frames,
            changed_frame: false,
        }
    }

    pub fn update(&mut self) {
        self.cur_frame_elapsed += if self.cur_frame_elapsed >= self.ticks_per_frame {
            self.cur_frame += 1;
            self.changed_frame = true;
            if self.cur_frame >= self.num_frames {
                self.cur_frame = 0;
            }
            self.cur_frame_elapsed = 0;
            0
        } else {
            self.changed_frame = false;
            1
        };
    }

    pub fn render(&self, p: impl Into<Point>) {
        self.spritesheet.render(self.cur_frame, p);
    }

    pub fn get_frame(&self) -> i32 {
        self.cur_frame
    }

    pub fn just_changed_frame(&self) -> bool {
        self.changed_frame
    }
}
