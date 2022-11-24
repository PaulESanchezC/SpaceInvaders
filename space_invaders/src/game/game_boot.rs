use std::io::{self, Stdout};

use crossterm::{
    cursor::{Hide, Show},
    terminal::{self, EnterAlternateScreen},
    ExecutableCommand,
};

pub fn start_game() -> Stdout {
    let mut stdout: Stdout = io::stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(Hide).unwrap();

    stdout
}

pub fn game_end(stdout: &mut Stdout) -> Result<(), ()> {
    terminal::enable_raw_mode().unwrap();
    stdout.execute(Show).unwrap();
    stdout.execute(EnterAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();

    Ok(())
}
