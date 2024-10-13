/**
 * Command-line Space Invaders, personal introduction to systems-level programming with Rust.
 * @author Maxylan (https://github.com/Maxylan)
 * @license MIT
 */
use std::{collections::LinkedList, time};
use termsize::Size;

mod entities;

#[derive(Debug)]
pub struct GameState {
    size: Size,
    enemies: LinkedList<entities::Alien>,
    projectiles: LinkedList<entities::Projectile>,
}

impl GameState {
    /** Create a new, default instance of GameState */
    pub fn new() -> Self {
        return GameState {
            size: Size {
                rows: 0_u16,
                cols: 0_u16,
            },
            enemies: LinkedList::new(),
            projectiles: LinkedList::new(),
        };
    }
    /**
     * Evaluates if its current state is correct, and adjusts accordingly.
     * Gets invoked on each iteration/loop.
     */
    pub fn evaluate_state(&mut self) -> Result<(), String> {
        if let Some(size) = termsize::get() {
            self.size = size;

            if self.size.cols < 8_u16 || self.size.rows < 8_u16 {
                return Err(String::from("Invalid terminal size ('termsize::get()')"));
            }
        } else {
            return Err(String::from(
                "Failed to compute terminal size ('termsize::get()')",
            ));
        }

        return Ok(());
    }
    /** Return all enemies (aliens) in the current GameState instance. */
    pub fn enemies(self) -> LinkedList<entities::Alien> {
        return self.enemies;
    }
}

#[derive(Debug)]
pub struct Arguments {
    pub frame_rate: Option<u8>,
    pub bullet_time: u8,
    pub enemy_time: u8,
}

pub fn start(args: Arguments) {
    let target_frame_time: f32 = match args.frame_rate {
        Some(rate) if rate > 0 => 1_f32 / rate as f32,
        _ => 1_f32 / u8::MAX as f32, // "Uncapped".
    };

    let mut state = GameState::new();
    let mut t = time::Instant::now();
    let mut meassure = 0_f32;
    loop {
        if meassure >= target_frame_time {
            // Clear the previous screen.
            if let Some(err) = clearscreen::clear().err() {
                println!("Cought an error calling 'clearscreen::clear()' : {}", err);
                return;
            }

            // Evaluate / Re-calculate game-state.
            // This validates enemy, player and projectile position in relation to current terminal size.
            state.evaluate_state();

            // Run an iteration of the game loop.
            game_loop((1_f32 / meassure).round() as u16);

            meassure = 0_f32;
            t = time::Instant::now();
            continue;
        }

        // Increment 'meassure' by elapsed time (..meassured in microseconds) in seconds.
        meassure += t.elapsed().subsec_micros() as f32 / 1000000_f32;
    }
}

fn construct_game_elements() {
    termsize::get().map(|size| println!("rows {} cols {}", size.rows, size.cols));
}

/**
 * Main game loop.
 * Runs capped to the specified framerate, with the actual framerate meassurement passed as an argument.
 */
fn game_loop(meassured_frame_rate: u16) {

    // Construct game elements.
}
