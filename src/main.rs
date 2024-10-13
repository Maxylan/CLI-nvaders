/**
 * Command-line Space Invaders, personal introduction to systems-level programming with Rust.
 * @author Maxylan (https://github.com/Maxylan)
 * @license MIT
 */
mod game;

fn main() {
    game::start(game::Arguments {
        frame_rate: Some(8),
        bullet_time: 2,
        enemy_time: 4,
    });
}
