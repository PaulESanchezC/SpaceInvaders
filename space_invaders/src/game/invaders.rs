use std::time::{Duration, Instant};

use super::{frame::Drawable, COLUMNS, ROWS};

pub struct Invader {
    x: usize,
    y: usize,
    alive: bool,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Instant,
    move_direction: i32,
}

impl Invader {
    pub fn new(x: usize, y: usize, alive: bool) -> Self {
        Self { x, y, alive }
    }
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..COLUMNS {
            for y in 0..ROWS {
                if x > 10 && x < COLUMNS - 11 && y > 4 && y < 15 && x % 3 == 0 && y % 3 == 0 {
                    army.push(Invader::new(x, y, true))
                }
            }
        }
        Self {
            army,
            move_timer: Instant::now(),
            move_direction: -1,
        }
    }

    pub fn move_army(&mut self) {
        if self.move_timer.elapsed() > Duration::from_millis(60) {
            let mut move_down = false;

            if self.move_direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.move_direction = 1;
                    move_down = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == COLUMNS - 1 {
                    self.move_direction = -1;
                    move_down = true;
                }
            }

            if move_down {
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.move_direction) as usize;
                }
            }
            self.move_timer = Instant::now();
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut super::frame::Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = "W";
        }
    }
}
