use crate::game_event::GameEvent;
use crate::game_object::{Drawable, Updatable};
use crate::texture_handler::TextureHandler;
use crate::util;

use std::error::Error;

use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct BasicLaser<'basic_laser, 'texture_handler> {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    rotation: f32,
    speed: f32,
    texture_offset_rotation: f32,
    texture: &'basic_laser Texture<'texture_handler>,
}

impl<'texture_handler> BasicLaser<'_, 'texture_handler> {
    pub fn new(
        texture_handler: &'texture_handler TextureHandler,
        x: f32,
        y: f32,
        rotation: f32,
    ) -> Self {
        Self {
            x,
            y,
            width: 20.,
            height: 20.,
            rotation,
            speed: 800.,
            texture_offset_rotation: 0.,
            texture: texture_handler.laser(),
        }
    }
}

impl<'texture_handler> Updatable<'texture_handler> for BasicLaser<'_, 'texture_handler> {
    fn update(
        &mut self,
        _: &'texture_handler TextureHandler,
        _: &crate::input_handler::InputHandler,
        delta_time: f32,
    ) -> Result<GameEvent<'texture_handler>, Box<dyn std::error::Error>> {
        let speed = self.speed * delta_time;
        self.x += speed * self.rotation.cos();
        self.y += speed * self.rotation.sin();
        Ok(GameEvent::None)
    }
}

impl<'texture_handler> Drawable<'texture_handler> for BasicLaser<'_, 'texture_handler> {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>> {
        canvas.copy_ex(
            self.texture,
            None,
            Some(util::rect(self.x, self.y, self.width, self.height)),
            (self.rotation + self.texture_offset_rotation).to_degrees() as f64,
            None,
            false,
            false,
        )?;
        Ok(())
    }
}
