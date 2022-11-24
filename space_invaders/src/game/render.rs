use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use super::{frame::Frame, ROWS};

pub fn render(
    stdout: &mut Stdout,
    last_frame: &Frame,
    new_frame: &Frame,
    force_render: bool,
    ammo_left: usize,
) {
    if force_render {
        stdout.queue(SetBackgroundColor(Color::DarkBlue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, column) in new_frame.iter().enumerate() {
        for (y, character) in column.iter().enumerate() {
            if *character != last_frame[x][y] || force_render {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *character);
            }
        }
    }
    stdout.queue(MoveTo(3, ROWS as u16 - 2)).unwrap();
    print!("Ammo: {}", ammo_left);
    stdout.flush().unwrap()
}
