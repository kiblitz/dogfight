use crate::camera::Camera;
use crate::game_event::GameEvent;
use crate::game_object::{Bounds, Drawable, GameTexture, Updatable};
use crate::input_handler::InputHandler;
use crate::texture_handler::TextureHandler;
use crate::util;

use std::error::Error;

pub struct BasicLaser<'basic_laser, 'texture_handler> {
    bounds: Bounds,
    rotation: f32,
    speed: f32,
    game_texture: GameTexture<'basic_laser, 'texture_handler>,
}

impl<'texture_handler> BasicLaser<'_, 'texture_handler> {
    pub fn new(
        texture_handler: &'texture_handler TextureHandler,
        x: f32,
        y: f32,
        rotation: f32,
    ) -> Self {
        Self {
            bounds: Bounds {
                x,
                y,
                width: 20.,
                height: 20.,
            },
            rotation,
            speed: 800.,
            game_texture: GameTexture {
                texture: texture_handler.laser(),
                rotation_offset: 0.,
            },
        }
    }
}

impl<'texture_handler> Updatable<'texture_handler> for BasicLaser<'_, 'texture_handler> {
    fn update(
        &mut self,
        _: &mut Camera,
        _: &'texture_handler TextureHandler,
        _: &InputHandler,
        delta_time: f32,
    ) -> Result<GameEvent<'texture_handler>, Box<dyn std::error::Error>> {
        let speed = self.speed * delta_time;
        self.bounds.x += speed * self.rotation.cos();
        self.bounds.y += speed * self.rotation.sin();
        Ok(GameEvent::None)
    }
}

impl<'texture_handler> Drawable<'texture_handler> for BasicLaser<'_, 'texture_handler> {
    fn draw(&self, camera: &mut Camera) -> Result<(), Box<dyn Error>> {
        camera.canvas_copy_ex(
            self.game_texture.texture,
            None,
            Some(util::rect(
                self.bounds.x,
                self.bounds.y,
                self.bounds.width,
                self.bounds.height,
            )),
            (self.rotation + self.game_texture.rotation_offset).to_degrees() as f64,
        )?;
        Ok(())
    }
}
