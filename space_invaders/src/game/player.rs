use super::{
    bullet_projectile::Bullet,
    frame::{Drawable, Frame},
    CLIP_SIZE, PLAYER_STRAFE_SPEED,
};
use crate::game::{COLUMNS, ROWS};

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub ammo: usize,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: COLUMNS / 2,
            y: ROWS - 10,
            ammo: CLIP_SIZE,
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= PLAYER_STRAFE_SPEED;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < COLUMNS - 1 {
            self.x += PLAYER_STRAFE_SPEED;
        }
    }

    pub fn move_down(&mut self) {
        if self.y < ROWS - 2 {
            self.y += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.y > 1 {
            self.y -= 2;
        }
    }

    pub fn shoot(&mut self) -> Option<Bullet> {
        if self.ammo > 0 {
            self.ammo -= 1;
            let bullet = Bullet::new(self.x, self.y - 1);
            return Some(bullet);
        }
        None
    }

    pub fn reload(&mut self) {
        self.ammo = CLIP_SIZE;
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
    }
}
