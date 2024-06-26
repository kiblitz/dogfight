use crate::game_event::GameEvent;
use crate::input_handler::InputHandler;
use crate::texture_handler::TextureHandler;

use std::error::Error;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Updatable<'texture_handler> {
    fn update(
        &mut self,
        texture_handler: &'texture_handler TextureHandler,
        input_handler: &InputHandler,
        delta_time: f32,
    ) -> Result<GameEvent<'texture_handler>, Box<dyn Error>>;
}

pub trait Drawable<'texture_handler>: Updatable<'texture_handler> {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>>;
}
