use std::time::{Duration, Instant};

use super::frame::{Drawable, Frame};

pub struct Bullet {
    x: usize,
    y: usize,
    impact: bool,
    delta: Instant,
}

impl Bullet {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            impact: false,
            delta: Instant::now(),
        }
    }

    pub fn refresh_position(&mut self) {
        if self.y == 1 {
            self.impact = true;
            return;
        }

        if self.y > 1 && self.delta.elapsed() > Duration::from_millis(20) {
            self.y -= 1;
            self.delta = Instant::now();
        }
    }

    // pub fn impacted(&mut self) {
    //     self.impact = true;
    // }

    pub fn is_dead(&self) -> bool {
        return self.impact || self.y == 1;
    }
}

impl Drawable for Bullet {
    fn draw(&self, frame: &mut Frame) {
        if self.impact {
            frame[self.x][self.y] = "O";
        } else {
            frame[self.x][self.y - 1] = "|";
        }
    }
}
