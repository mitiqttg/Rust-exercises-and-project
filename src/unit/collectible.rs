use crate::{point::Point2d, traits::Position};

#[derive(Debug, Default)]
pub struct Collectible {
    position: Point2d<u16>,
}

impl Position<u16> for Collectible {
    fn position(&self) -> Point2d<u16> {
        self.position
    }
    fn set_position(&mut self, position: Point2d<u16>) {
        self.position = position;
    }
}