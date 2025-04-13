use crate::{
    hud::Hud,
    traits::Position,
    unit::{Collectible, Enemy, Player, Wall},
};
use std::{fmt::Display, io::Write};
use crossterm::{style::{ Stylize},};
use num::{traits::NumAssign, NumCast};

impl Display for Collectible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", '❤'.red())
    }
}

impl Display for Wall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", '🧱')
    }
}

impl Display for Enemy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", '👾')
    }
}

impl<'a> Display for Hud<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

pub trait Draw<T: NumAssign + Copy + NumCast>: Position<T> + Display {
    fn draw(&self, stdout: &mut impl Write) {
        let position = self.position();
        crossterm::queue!(
            stdout,
            crossterm::cursor::MoveTo(
                position
                    .x
                    .to_f64()
                    .expect("could not convert position x to f64")
                    .round() as u16,
                position
                    .y
                    .to_f64()
                    .expect("could not convert position y to f64")
                    .round() as u16,
            ),
            crossterm::style::Print(self)
        )
        .unwrap();
    }
}

impl Draw<f64> for Player {}
impl Draw<u16> for Collectible {}
impl Draw<u16> for Wall {}
impl Draw<f64> for Enemy {}
impl Draw<u16> for Hud<'_> {}