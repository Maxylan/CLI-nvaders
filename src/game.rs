use core::panic;
/**
 * Command-line Space Invaders, personal introduction to systems-level programming with Rust.
 * @author Maxylan (https://github.com/Maxylan)
 * @license MIT
 */
use std::{collections::LinkedList, error::Error, time};
use termsize::Size;

mod entities;

#[derive(Debug)]
pub struct GameState {
    size: Size,
    enemies: LinkedList<entities::Alien>,
    projectiles: LinkedList<entities::Projectile>,
    falling_stars: LinkedList<entities::FallingStar>,
    player: entities::Player,
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
            falling_stars: LinkedList::new(),
            player: entities::Player { pos: 0_u16 },
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

        // TODO! Process game state..

        return Ok(());
    }
    /** Return all enemies (aliens) in the current GameState instance. */
    pub fn enemies(self) -> LinkedList<entities::Alien> {
        return self.enemies;
    }
    /** Return all projectiles in the current GameState instance. */
    pub fn projectiles(self) -> LinkedList<entities::Projectile> {
        return self.projectiles;
    }
    /** Return all projectiles in the current GameState instance. */
    pub fn player(self) -> entities::Player {
        return self.player;
    }
}

pub struct Arguments {
    pub frame_rate: Option<u8>,
    pub bullet_time: u8,
    pub enemy_time: u8,
    pub panic_on_errors: bool,
}

/**
 * Starts the game by initiating the loop.
 */
pub fn start(args: Arguments) {
    let target_frame_time: f32 = match args.frame_rate {
        Some(rate) if rate > 0 => 1_f32 / rate as f32,
        _ => 1_f32 / u8::MAX as f32, // "Uncapped".
    };

    let mut state = GameState::new();
    state.evaluate_state().expect(
        "Failed to start! 'GameState::evaluate_state()' paniced! Are you running TempleOS?",
    );

    // Populate state..
    state.falling_stars.push_back(entities::FallingStar { pos: 0, col: 0 });

    // Shift 'size.rows' to effectively 'half'-it, determining player's starting position.
    state.player.pos = state.size.rows >> 1_u8;

    let mut t = time::Instant::now();
    let mut frame_time = 0_f32;
    loop {
        if frame_time >= target_frame_time {
            let meassure: u16 = (1_f32 / frame_time).round() as u16;
            t = time::Instant::now();
            frame_time = 0_f32;

            // Run an iteration of the game loop.
            let frame_execution_result = game_loop(&mut state);
            match frame_execution_result {
                Ok(_) => render(meassure, &state),
                Err(error_message) => {
                    if args.panic_on_errors {
                        panic!("Panic! {}", error_message);
                    } else {
                        println!("Error! {}", error_message);
                    }
                }
            }

            continue;
        }

        // Increment 'meassure' by elapsed time (..meassured in microseconds) in seconds.
        frame_time += t.elapsed().subsec_micros() as f32 / 1000000_f32;
    }
}

/**
 * Main game loop.
 * Runs capped to the specified framerate, with the actual framerate meassurement passed as an argument.
 */
fn game_loop(state: &mut GameState) -> Result<(), String> {
    // Clear the previous screen.
    if let Err(e) = clearscreen::clear() {
        return Err(format!(
            "Cought an error calling 'clearscreen::clear()', {e}"
        ));
    }

    // Evaluate / Re-calculate game-state.
    // This validates enemy, player and projectile position in relation to current terminal size.
    state.evaluate_state()?;

    return Ok(());
}

fn render(frame_rate: u16, state: &GameState) {
    println!();
    let mut line: String;
    // Line #1 - Debugging / Messaging
    let message = right_pad(format!("Framerate: {frame_rate}"), state.size.rows.into());
    println!("{}", message);

    let mut start_at_row = 1;
    if state.size.rows > 10 {
        println!("{}", "=".repeat(state.size.rows as usize));
        start_at_row += 1;
    }

    // TODO! Rest of rendering lines..
}

/**
 * Pad-out the string length to fill out the remaining cells of a row.
 */
fn right_pad(mut string_content: String, length: usize) -> String {
    let char_count: usize = string_content.chars().count();

    if char_count == length {
        return string_content;
    } else if char_count > length {
        return string_content.split_at(length).0.into();
    };

    string_content += &" ".repeat(length - char_count);
    return string_content;
}

/**
 * ..I don't need no NPM Package!
 */
fn left_pad(start_index: u16, mut string_content: String, length: u16) -> String {
    if start_index >= length {
        return vec![" "; length as usize].join("");
    };

    let line: String = match start_index {
        0 => String::new(),
        // _ => vec![" "; start_index as usize].join(""),
        _ => " ".repeat(start_index.into())
    };

    string_content.insert_str(0, &line); 
    return string_content;
}
