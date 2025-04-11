use crate::{point::Point2d, traits::Position};

#[derive(Debug, Default)]
pub struct Wall {
    position: Point2d<u16>,
}

impl Wall {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            position: Point2d::new(x, y),
        }
    }
}

impl Position<u16> for Wall {
    fn position(&self) -> Point2d<u16> {
        self.position
    }
    fn set_position(&mut self, position: Point2d<u16>) {
        self.position.x = position.x;
        self.position.y = position.y;
    }
}
