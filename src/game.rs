use core::panic;
/**
 * Command-line Space Invaders, personal introduction to systems-level programming with Rust.
 * @author Maxylan (https://github.com/Maxylan)
 * @license MIT
 */
use std::{collections::LinkedList, error::Error, time};
use termsize::Size;
use rand::Rng;

mod entities;

#[derive(Debug)]
pub struct GameState {
    size: Size,
    enemies: Vec<entities::Alien>,
    projectiles: Vec<entities::Projectile>,
    falling_stars: Vec<entities::FallingStar>,
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
            enemies: vec!(),
            projectiles: vec!(),
            falling_stars: vec!(),
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
    pub fn enemies(self) -> Vec<entities::Alien> {
        return self.enemies;
    }
    /** Return all projectiles in the current GameState instance. */
    pub fn projectiles(self) -> Vec<entities::Projectile> {
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
    let mut rng = rand::thread_rng();
    while state.falling_stars.len() < 4 { // Testing 4 stars!
        let col: u16 = rng.gen_range(1..state.size.cols);
        for star in state.falling_stars.iter() {
            if star.col == col {
                continue;
            }
        }

        state.falling_stars.push(entities::FallingStar { pos: 0, col });
    }

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
    let mut message = right_pad(format!("Framerate: {frame_rate}"), state.size.rows.into());
    println!("{}", message);

    let start_at_row: u16 = if state.size.rows > 10 {
        println!("{}", "=".repeat(state.size.rows as usize));
        2
    } else {
        1
    };

    let mut current_row = start_at_row;
    while current_row < state.size.rows {
        let mut start_at_col: Option<u16> = None; // Determine what the first 'col' is for this row
        let mut star_indicies: Vec<u8> = vec![]; // Effectively 'filters' falling-stars

        for star in &state.falling_stars {
            *star_indicies.last_mut().unwrap() += 1;

            if star.pos != current_row {
                continue; // The star is not on the current row..
            }

            if start_at_col.is_none() || star.col - 1 < start_at_col.unwrap() {
                start_at_col = Some(star.col - 1);
            }

            star_indicies.push(*star_indicies.last().unwrap());
        }

        message = "=".repeat(state.size.rows as usize);
        if start_at_col.is_none() {
            println!("{}", message);
            current_row += 1;
            continue;
        }

        for index in star_indicies {
            replace_at(&mut message, state.falling_stars[index as usize].entity, state.falling_stars[index as usize].col)
        }
    }
}

fn replace_at(content: &mut String, character: char, index: u16) -> () {
    content.replace_range(
        content
            .char_indices()
            .nth(index.into())
            .map(|(i, c)| (i..i + c.len_utf8()))
            .unwrap(),
        &character.to_string()
    )
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
