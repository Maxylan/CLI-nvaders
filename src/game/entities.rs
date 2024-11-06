/**
 * Command-line Space Invaders, personal introduction to systems-level programming with Rust.
 * @author Maxylan (https://github.com/Maxylan)
 * @license MIT
 */
#[derive(Debug)]
pub struct Player {
    pub pos: u16,
}

#[derive(Debug)]
pub struct FallingStar {
    pub pos: u16,
    pub col: u16,
    pub entity: char,
}

#[derive(Debug)]
pub struct Projectile {}

#[derive(Debug)]
pub struct Alien {}
