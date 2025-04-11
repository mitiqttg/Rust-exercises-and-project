use std::{
    io::{stdout, Stdout, Write},
    time::{Duration},
};

use rand::rngs::ThreadRng;
use crossterm::queue;
use crossterm::event::KeyCode;

use crate::{
    hud::Hud,
    input,
    traits::*,
    ui::{draw::*, UI},
    unit::Collectible,
    unit::Enemy,
    unit::Wall,
    unit::{Player, PlayerBuilder},
};

pub struct Game {
    height: u16,
    width: u16,
    stdout: Stdout,
    score: u32,
    enemies: Vec<Enemy>,
    n_random_walls: u16,
    walls: Vec<Wall>,
    collectible: Collectible,
    player: Player,
    player_builder: PlayerBuilder,
    ui: UI,
    rng: ThreadRng,
    update_interval_millis: Duration,
}

pub struct GameBuilder {
    height: u16,
    width: u16,
    n_random_walls: u16,
    update_interval: Duration,
    player_builder: PlayerBuilder,
    enemies: Vec<Enemy>,
    walls: Vec<Wall>,
}

impl GameBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            height: 48,
            width: 80,
            player_builder: PlayerBuilder::new(),
            n_random_walls: 0,
            update_interval: Duration::from_millis(50),
            enemies: vec![
                Enemy::with_speed(0.1),
                Enemy::with_speed(0.5),
                Enemy::with_speed(0.8),
            ],
            walls: vec![],
        }
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    pub fn player_starting_health(mut self, health: u8) -> Self {
        self.player_builder = self.player_builder.health(health);
        self
    }

    pub fn player_starting_speed(mut self, speed: f64) -> Self {
        self.player_builder = self.player_builder.speed(speed);
        self
    }

    pub fn n_random_walls(mut self, n_random_walls: u16) -> Self {
        self.n_random_walls = n_random_walls;
        self
    }

    pub fn update_interval(mut self, update_interval: Duration) -> Self {
        self.update_interval = update_interval;
        self
    }

    pub fn enemies(mut self, enemies: Vec<Enemy>) -> Self {
        self.enemies = enemies;
        self
    }

    pub fn walls(mut self, walls: Vec<Wall>) -> Self {
        self.walls = walls;
        self
    }

    pub fn build(self) -> Game {
        let player_builder = self.player_builder.clone(); // Clone the builder
        Game {
            height: self.height,
            width: self.width,
            n_random_walls: self.n_random_walls,
            update_interval_millis: self.update_interval,
            enemies: self.enemies,
            walls: self.walls,
            collectible: Collectible::default(),
            player: self.player_builder.build(), // This consumes the original
            player_builder, // Use the clone here
            ui: UI::new(),
            rng: rand::rngs::ThreadRng::default(),
            stdout: stdout(),
            score: 0,
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn init(&mut self) {
        self.ui.prepare();
        self.walls.clear();

        for x in 0..self.width {
            self.walls.push(Wall::new(x, 0));
            self.walls.push(Wall::new(x, self.height - 1));
        }
        for y in 0..self.height {
            self.walls.push(Wall::new(0, y));
            self.walls.push(Wall::new(self.width - 1, y));
        }

        // add random walls
        for _ in 0..self.n_random_walls {
            let mut wall = Wall::default();
            wall.set_rand_position(&mut self.rng, 1..self.width - 1, 1..self.height - 1);
            self.walls.push(wall);
        }

        // randomize enemy positions
        self.enemies.clear(); // Ensure enemies are cleared before re-generating
        self.enemies.extend(
            (1..12) 
                .map(|i| Enemy::with_speed(i as f64 * 0.05))
                .collect::<Vec<Enemy>>()
        );
        self.enemies.iter_mut().for_each(|enemy| {
            enemy.set_rand_position(
                &mut self.rng,
                1.0..(self.width - 1).into(),
                1.0..(self.height - 1).into(),
            );
        });

        // randomize collectible position
        self.collectible = Collectible::default(); // Reset the collectible
        while self
            .walls
            .iter()
            .any(|wall| wall.position() == self.collectible.position())
        {
            self.collectible.set_rand_position(
                &mut self.rng,
                1..self.width - 1,
                1..self.height - 1,
            );
        }
    }

    fn update(&mut self) {
        // move player if not colliding with a wall
        let player_next_position = self.player.forward_position();
        if !self
            .walls
            .iter()
            .any(|wall| wall.position() == player_next_position)
        {
            self.player.move_forward();
        }

        // increase score if player collides with collectible
        if self.player.position().round().to_u16() == self.collectible.position() {
            self.score += 1;
            // move collectible to a new random position
            self.collectible.set_rand_position(
                &mut self.rng,
                1..self.width - 1,
                1..self.height - 1,
            );
            while self
                .walls
                .iter()
                .any(|wall| wall.position() == self.collectible.position())
            {
                self.collectible.set_rand_position(
                    &mut self.rng,
                    1..self.width - 1,
                    1..self.height - 1,
                );
            }
        }

        // move enemies
        self.enemies
            .iter_mut()
            .for_each(|enemy| enemy.move_towards_player(&self.player));

        // reduce player health for each enemy collision
        self.enemies.iter_mut().for_each(|enemy| {
            if enemy.position().round() == self.player.position().round() {
                self.player.take_damage(1);
            }
        });
    }

    fn draw(&mut self) {
        self.ui.clear();
        let mut buffer: Vec<u8> = Vec::new();

        // Draw game elements within game bounds
        self.walls.iter().for_each(|wall| wall.draw(&mut buffer));
        self.player.draw(&mut buffer);
        self.enemies.iter().for_each(|enemy| enemy.draw(&mut buffer));
        self.collectible.draw(&mut buffer);

        // Draw HUD below game area with a separator line
        let separator_y = self.height;
        queue!(
            buffer,
            crossterm::cursor::MoveTo(0, separator_y),
            crossterm::style::Print("─".repeat(self.width as usize))
        ).unwrap();

        // Ensure the HUD's y_position is set correctly relative to the separator
        let hud_y = separator_y + 1;
        Hud::new(self.score, &self.player, hud_y).draw(&mut buffer);

        self.stdout.write_all(&buffer).expect("write failed");
        self.stdout.flush().expect("flush failed");
    }
    pub fn reset(&mut self) {
        self.score = 0;
        self.player = self.player_builder.clone().build(); // Clone and build player
        self.rng = rand::rngs::ThreadRng::default();
        self.init(); // Call init to regenerate walls, enemies, and collectible
    }

    pub fn run(&mut self) {
        loop {
            self.init();
            let mut quit = false;
            while self.player.is_alive() && !quit {
                let now = std::time::Instant::now();
                while let Some(time_remaining) = self.update_interval_millis.checked_sub(now.elapsed()) {
                    if let Some(key) = input::poll_key_event(time_remaining) {
                        input::handle_key_event(key, &mut self.player, &mut quit);
                    }
                }

                self.update();
                self.draw();
            }

            self.ui.restore();
            println!("\nGame over! Score: {}", self.score);
            println!("Press R(r) to restart or Q(q) to quit");

            // Wait for restart or quit
            loop {
                if let Some(key) = input::poll_key_event(Duration::from_secs(1)) {
                    match key.code {
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            self.reset();
                            break;
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return,
                        _ => {}
                    }
                }
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::builder().build()
    }
}