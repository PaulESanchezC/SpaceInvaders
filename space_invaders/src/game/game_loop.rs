use crossterm::event::{self, Event, KeyCode};
use std::{io::Stdout, time::Duration};

use super::{
    bullet_projectile::Bullet,
    frame::{new_frame, Drawable},
    invaders::Invaders,
    player::Player,
    render::render,
    CLIP_SIZE,
};

pub fn game_loop(stdout: &mut Stdout) {
    let mut player = Player::new();
    let mut invaders = Invaders::new();
    let first_frame = new_frame();

    render(stdout, &first_frame, &first_frame, true, CLIP_SIZE);

    let mut current_frame = first_frame;
    let mut game_bullets: Vec<Bullet> = Vec::new();

    'gameLoop: loop {
        let mut new_frame = new_frame();
        while event::poll(Duration::default()).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameLoop;
                    }
                    KeyCode::Enter => {
                        println!("pause")
                    }
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Up => player.move_up(),
                    KeyCode::Down => player.move_down(),
                    KeyCode::Char('a') => {
                        let bullet = player.shoot();
                        match bullet {
                            Some(b) => {
                                game_bullets.push(b);
                            }
                            None => {}
                        }
                    }
                    KeyCode::Char('r') => player.reload(),
                    _ => {}
                }
            }
        }

        game_bullets.retain(|bullet| !bullet.is_dead());

        for bullet in game_bullets.iter_mut() {
            bullet.refresh_position();
            bullet.draw(&mut new_frame);
        }

        player.draw(&mut new_frame);

        invaders.move_army();
        invaders.draw(&mut new_frame);

        render(stdout, &current_frame, &new_frame, false, player.ammo);
        current_frame = new_frame;
    }
}
