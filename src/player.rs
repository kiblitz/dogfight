use crate::event_handler::EventHandler;
use crate::game_object::{Drawable, Updatable};
use crate::texture_handler::TextureHandler;
use crate::util;

use std::error::Error;
use std::f32::consts::PI;

use glam::Vec2;

use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct Player<'player, 'texture_handler> {
    x: f32,
    y: f32,
    speed: Vec2,
    acceleration: f32,
    drag_constant: f32,
    break_drag_constant: f32,
    rotation: f32,
    rotation_speed: f32,
    rotation_acceleration: f32,
    rotation_drag_constant: f32,
    texture: &'player Texture<'texture_handler>,
    texture_offset_rotation: f32,
}

impl Updatable for Player<'_, '_> {
    fn update(
        &mut self,
        event_handler: &EventHandler,
        delta_time: f32,
    ) -> Result<(), Box<dyn Error>> {
        if *event_handler.up() && !event_handler.down() {
            self.speed += Vec2::from_angle(self.rotation) * self.acceleration * delta_time;
        } else if *event_handler.down() && !event_handler.up() {
            self.speed = self.speed.normalize_or_zero()
                * util::drag(self.speed.length(), self.break_drag_constant, delta_time);
        }
        self.speed = self.speed.normalize_or_zero()
            * util::drag(self.speed.length(), self.drag_constant, delta_time);

        if *event_handler.left() {
            self.rotation_speed -= self.rotation_acceleration * delta_time;
        }
        if *event_handler.right() {
            self.rotation_speed += self.rotation_acceleration * delta_time;
        }
        self.rotation_speed =
            util::drag(self.rotation_speed, self.rotation_drag_constant, delta_time);

        self.x += self.speed.x * delta_time;
        self.y += self.speed.y * delta_time;
        self.rotation += self.rotation_speed * delta_time;

        Ok(())
    }
}

impl Drawable for Player<'_, '_> {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>> {
        let (_width, _height) = canvas.output_size()?;

        let (x, y) = self.pos();
        let rect = Rect::new(x as i32, y as i32, 100, 100);
        canvas.copy_ex(
            self.texture,
            None,
            Some(rect),
            (self.rotation + self.texture_offset_rotation).to_degrees() as f64,
            None,
            false,
            false,
        )?;
        Ok(())
    }
}

impl<'texture_handler> Player<'_, 'texture_handler> {
    pub fn new(texture_handler: &'texture_handler TextureHandler) -> Self {
        Self {
            x: 0.,
            y: 0.,
            speed: Vec2::ZERO,
            acceleration: 420.,
            drag_constant: 0.6,
            break_drag_constant: 0.3,
            rotation: 0.,
            rotation_speed: 0.,
            rotation_acceleration: 12.,
            rotation_drag_constant: 0.15,
            texture: texture_handler.player(),
            texture_offset_rotation: PI / 2.,
        }
    }

    pub fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
