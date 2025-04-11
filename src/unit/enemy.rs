use crate::unit::Player;
use crate::{point::Point2d, traits::Position};

#[derive(Debug, Default)]
pub struct Enemy {
    position: Point2d<f64>,
    speed: f64,
}

impl Enemy {
    pub fn with_speed(speed: f64) -> Self {
        Self {
            position: Point2d::new(0.0, 0.0),
            speed,
        }
    }

    pub fn move_towards_player(&mut self, player: &Player) {
        let direction = player.position().round() - self.position().round();

        self.position += direction.normalize() * self.speed;
    }
}

impl Position<f64> for Enemy {
    fn position(&self) -> Point2d<f64> {
        self.position
    }
    fn set_position(&mut self, position: Point2d<f64>) {
        self.position.x = position.x;
        self.position.y = position.y;
    }
}
