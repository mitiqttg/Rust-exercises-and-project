use std::{
    f64::consts::PI,
    fmt::{Display, Formatter},
};

use crate::{point::Point2d, traits::Position};

#[derive(Debug, Default)]
pub struct Player {
    position: Point2d<f64>,
    direction: Point2d<f64>,
    speed: f64,
    health: u8,
}

impl Position<f64> for Player {
    fn position(&self) -> Point2d<f64> {
        self.position
    }
    fn set_position(&mut self, position: Point2d<f64>) {
        self.position = position;
    }
}

impl Player {
    pub fn new(position: Point2d<f64>, direction: Point2d<f64>, speed: f64, health: u8) -> Self {
        Self {
            position,
            direction,
            speed,
            health,
        }
    }

    pub fn builder() -> PlayerBuilder {
        PlayerBuilder::new()
    }

    pub fn forward_position(&self) -> Point2d<u16> {
        (self.position + (self.direction * self.speed))
            .round()
            .to_u16()
    }

    pub fn move_forward(&mut self) {
        self.position += self.direction * self.speed;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn take_damage(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }

    pub fn health(&self) -> u8 {
        self.health
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }

    pub fn face_up(&mut self) {
        self.direction = Point2d::new(0.0, -1.0);
    }

    pub fn face_down(&mut self) {
        self.direction = Point2d::new(0.0, 1.0);
    }
    
    pub fn turn_left(&mut self) {
        let angle = PI / 4.0;
        self.direction = self.direction.rotate(angle);
    }

    pub fn turn_right(&mut self) {
        let angle = -PI / 4.0;
        self.direction = self.direction.rotate(angle);
    }

    pub fn accelerate(&mut self) {
        self.speed += 0.1;
        self.speed = self.speed.min(1.0);
    }

    pub fn decelerate(&mut self) {
        self.speed -= 0.1;
        self.speed = self.speed.max(0.0);
    }
}

#[derive(Clone)] 
pub struct PlayerBuilder {
    position: Point2d<f64>,
    direction: Point2d<f64>,
    speed: f64,
    health: u8,
}

#[allow(clippy::new_without_default)]
impl PlayerBuilder {
    pub fn new() -> Self {
        Self {
            position: Point2d::new(1.0, 1.0),
            direction: Point2d::new(1.0, 0.0),
            speed: 0.0,
            health: 10,
        }
    }

    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.position = Point2d::new(x, y);
        self
    }

    pub fn direction(mut self, x: f64, y: f64) -> Self {
        self.direction = Point2d::new(x, y);
        self
    }

    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = speed;
        self
    }

    pub fn health(mut self, health: u8) -> Self {
        self.health = health;
        self
    }

    pub fn build(self) -> Player {
        Player {
            position: self.position,
            direction: self.direction,
            speed: self.speed,
            health: self.health,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crossterm::style::Stylize;

        let icon = if self.speed == 0.0 {
            "😴"
        } else {
            match self.direction.round().to_i16() {
                Point2d { x: 0, y: -1 } => "↑",
                Point2d { x: 1, y: -1 } => "↗",
                Point2d { x: 1, y: 0 } => "→",
                Point2d { x: 1, y: 1 } => "↘",
                Point2d { x: 0, y: 1 } => "↓",
                Point2d { x: -1, y: 1 } => "↙",
                Point2d { x: -1, y: 0 } => "←",
                Point2d { x: -1, y: -1 } => "↖",
                _ => "😴",
            }
        };
        write!(f, "{}", icon.yellow())
    }
}
