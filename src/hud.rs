use crate::{
    point::Point2d, 
    traits::Position,
    unit::Player  // Add this import
};
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{PrintStyledContent, Stylize},
};
use std::io::Write;

pub struct Hud<'a> {
    score: u32,
    player: &'a Player,
    y_position: u16,
}

impl<'a> Position<u16> for Hud<'a> {
    fn position(&self) -> Point2d<u16> {
        Point2d::new(0, self.y_position)
    }
    
    fn set_position(&mut self, position: Point2d<u16>) {
        self.y_position = position.y;
    }
}

impl<'a> Hud<'a> {
    pub fn new(score: u32, player: &'a Player, y_position: u16) -> Self {
        Hud {
            score,
            player,
            y_position,
        }
    }

    pub fn draw(&self, buffer: &mut impl Write) {
        let health_text = format!("Health: {}", self.player.health()).red();
        let speed_text = format!("Speed: {:.1}", self.player.speed()).blue();
        let score_text = format!("Score: {}", self.score).green();

        queue!(
            buffer,
            MoveTo(0, self.y_position),
            PrintStyledContent(" STATUS ".on_dark_grey()),
            MoveTo(10, self.y_position),
            PrintStyledContent(health_text),
            MoveTo(25, self.y_position),
            PrintStyledContent(speed_text),
            MoveTo(40, self.y_position),
            PrintStyledContent(score_text),
        ).unwrap();
    }

    pub fn text(&self) -> String {
        format!(
            "Health: {} | Speed: {:.1} | Score: {}",
            self.player.health(),
            self.player.speed(),
            self.score
        )
    }
}