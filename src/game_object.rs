use crate::camera::Camera;
use crate::game_event::GameEvent;
use crate::input_handler::InputHandler;
use crate::texture_handler::TextureHandler;

use std::error::Error;

use sdl2::render::Texture;

#[derive(Clone, Copy)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct GameTexture<'owner, 'texture_handler> {
    pub texture: &'owner Texture<'texture_handler>,
    pub rotation_offset: f32,
}

pub trait Updatable<'texture_handler> {
    fn update(
        &mut self,
        camera: &mut Camera,
        texture_handler: &'texture_handler TextureHandler,
        input_handler: &InputHandler,
        delta_time: f32,
    ) -> Result<GameEvent<'texture_handler>, Box<dyn Error>>;
}

pub trait Drawable<'texture_handler>: Updatable<'texture_handler> {
    fn draw(&self, camera: &mut Camera) -> Result<(), Box<dyn Error>>;
}
