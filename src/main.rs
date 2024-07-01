extern crate sdl2;

mod basic_laser;
mod camera;
mod dogfight;
mod game;
mod game_event;
mod game_object;
mod input_handler;
mod player;
mod texture_handler;
mod timer_object;
mod util;

use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let result = dogfight::run_game();
    if let Err(ref _e) = result {
        // TODO handle error logging
    };
    result
}
