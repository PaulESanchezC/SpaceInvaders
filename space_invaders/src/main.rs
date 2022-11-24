mod game;
use game::game_boot;
use game::game_loop;

fn main() -> Result<(), ()> {
    let mut game = game_boot::start_game();

    game_loop::game_loop(&mut game);

    let game_end = game_boot::game_end(&mut game);
    game_end
}
