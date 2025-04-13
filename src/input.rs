use std::time::Duration;
use crate::unit::Player;
use crossterm::event::{poll, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind};


pub fn poll_key_event(duration: Duration) -> Option<KeyEvent> {
    if poll(duration).ok()? {
        let event = crossterm::event::read().ok()?;
        if let Event::Key(key_event) = event {
            if key_event.kind == KeyEventKind::Press {
                return Some(key_event);
            }
        }
    }
    None
}

pub fn handle_key_event(key: KeyEvent, player: &mut Player, quit: &mut bool) {
    match key.code {
        KeyCode::Left => player.turn_left(),
        KeyCode::Right => player.turn_right(),
        KeyCode::Up => player.accelerate(),
        KeyCode::Down => player.decelerate(),
        KeyCode::Char('q') | KeyCode::Esc => *quit = true,
        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => *quit = true,
        _ => {}
    }
}
