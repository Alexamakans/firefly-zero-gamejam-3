use crate::game::position::Position;

pub struct Camera {
    pub pos: Position,
}

impl Camera {
    pub fn new(pos: Position) -> Self {
        Self { pos }
    }
}
