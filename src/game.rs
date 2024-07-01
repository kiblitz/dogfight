use crate::camera::Camera;
use crate::game_event::GameEvent;
use crate::game_object::{Drawable, Updatable};
use crate::input_handler::InputHandler;
use crate::player::Player;
use crate::texture_handler::TextureHandler;

use std::collections::LinkedList;
use std::error::Error;
use std::num::Wrapping;

use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct Game<'game, 'texture_handler> {
    frame: Wrapping<u8>,
    camera: Camera,
    player: Player<'game, 'texture_handler>,
    background: &'game Texture<'texture_handler>,
    entities: LinkedList<Box<dyn Drawable<'texture_handler> + 'texture_handler>>,
}

impl<'texture_handler> Game<'_, 'texture_handler> {
    pub fn new(canvas: Canvas<Window>, texture_handler: &'texture_handler TextureHandler) -> Self {
        Self {
            frame: Wrapping(0u8),
            camera: Camera::new(canvas, -100., -100., 600., 400.),
            player: Player::new(texture_handler),
            background: texture_handler.stars_background(),
            entities: LinkedList::new(),
        }
    }

    pub fn frame(&self) -> u8 {
        self.frame.0
    }

    pub fn update(
        &mut self,
        texture_handler: &'texture_handler TextureHandler,
        input_handler: &InputHandler,
        delta_time: f32,
    ) -> Result<GameEvent<'texture_handler>, Box<dyn Error>> {
        self.frame += Wrapping(1u8);
        match self.player.update(
            &mut self.camera,
            texture_handler,
            &input_handler,
            delta_time,
        )? {
            GameEvent::PlayerShoot(basic_laser) => self.entities.push_back(basic_laser),
            _ => (),
        }
        for entity in self.entities.iter_mut() {
            entity.update(&mut self.camera, texture_handler, input_handler, delta_time)?;
        }
        self.camera.update(delta_time);
        Ok(GameEvent::None)
    }

    pub fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        let i = self.frame();
        self.camera.set_canvas_draw_color(Color::RGB(
            if i < 128 { i } else { 255 - i },
            64,
            128 - if i < 128 { i } else { 255 - i },
        ));
        self.camera.canvas_clear();
        self.camera
            .canvas_copy_ex(self.background, None, None, 0.)?;

        for entity in self.entities.iter() {
            entity.draw(&mut self.camera)?;
        }
        self.player.draw(&mut self.camera)?;
        self.camera.canvas_present();
        Ok(())
    }
}
