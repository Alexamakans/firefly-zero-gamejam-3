use core::ops;

use firefly_rust::{Point, Size, math::sqrt};

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + (b - a) * t
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn lerp(&mut self, t: f32, to: Position) {
        self.x = lerp(t, self.x, to.x);
        self.y = lerp(t, self.y, to.y);
    }

    pub fn clamp_to(&mut self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) {
        if self.x < min_x {
            self.x = min_x;
        }
        if self.y < min_y {
            self.y = min_y;
        }
        if self.x > max_x {
            self.x = max_x;
        }
        if self.y > max_y {
            self.y = max_y;
        }
    }

    pub fn dist_sq(&self, rhs: &Position) -> f32 {
        (self.x - rhs.x) * (self.x - rhs.x) + (self.y - rhs.y) * (self.y - rhs.y)
    }

    pub fn dist(&self, rhs: &Position) -> f32 {
        sqrt(self.dist_sq(rhs))
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        // good enoguh?
        self.x as i32 == other.x as i32 && self.y as i32 == other.y as i32
    }
}
impl Eq for Position {}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(mut self, rhs: Position) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(mut self, rhs: Position) -> Self::Output {
        self -= rhs;
        self
    }
}

impl ops::SubAssign<Position> for Position {
    fn sub_assign(&mut self, rhs: Position) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<Position> for Position {
    type Output = Position;

    fn mul(mut self, rhs: Position) -> Self::Output {
        self *= rhs;
        self
    }
}

impl ops::MulAssign<Position> for Position {
    fn mul_assign(&mut self, rhs: Position) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl ops::Mul<f32> for Position {
    type Output = Position;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl ops::MulAssign<f32> for Position {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Div<f32> for Position {
    type Output = Position;

    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}

impl ops::DivAssign<f32> for Position {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl From<&Position> for Point {
    fn from(value: &Position) -> Point {
        Point {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

impl From<Position> for Point {
    fn from(value: Position) -> Self {
        Self::from(&value)
    }
}

impl From<&Point> for Position {
    fn from(value: &Point) -> Self {
        Position {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl From<Point> for Position {
    fn from(value: Point) -> Self {
        Self::from(&value)
    }
}

impl From<&Size> for Position {
    fn from(value: &Size) -> Self {
        Position {
            x: value.width as f32,
            y: value.height as f32,
        }
    }
}

impl From<Size> for Position {
    fn from(value: Size) -> Self {
        Self::from(&value)
    }
}
