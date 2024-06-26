use crate::basic_laser::BasicLaser;
use crate::game_event::GameEvent;
use crate::game_object::{Drawable, Updatable};
use crate::input_handler::InputHandler;
use crate::texture_handler::TextureHandler;
use crate::timer_object::TimerObject;
use crate::util;

use std::error::Error;
use std::f32::consts::PI;
use std::time::Duration;

use glam::Vec2;

use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

#[derive(derive_getters::Getters)]
pub struct Player<'player, 'texture_handler> {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
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
    fire_timer: TimerObject,
    fire_ready: bool,
}

impl<'texture_handler> Player<'_, 'texture_handler> {
    pub fn new(texture_handler: &'texture_handler TextureHandler) -> Self {
        Self {
            x: 0.,
            y: 0.,
            width: 100.,
            height: 100.,
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
            fire_timer: TimerObject::new(Duration::from_millis(200).as_secs_f32()),
            fire_ready: true,
        }
    }
}

impl<'texture_handler> Updatable<'texture_handler> for Player<'_, 'texture_handler> {
    fn update(
        &mut self,
        texture_handler: &'texture_handler TextureHandler,
        input_handler: &InputHandler,
        delta_time: f32,
    ) -> Result<GameEvent<'texture_handler>, Box<dyn Error>> {
        match self
            .fire_timer
            .update(texture_handler, input_handler, delta_time)?
        {
            GameEvent::TimerFire => {
                self.fire_ready = true;
            }
            _ => (),
        };

        // movement
        if *input_handler.up() && !input_handler.down() {
            self.speed += Vec2::from_angle(self.rotation) * self.acceleration * delta_time;
        } else if *input_handler.down() && !input_handler.up() {
            self.speed = self.speed.normalize_or_zero()
                * util::drag(self.speed.length(), self.break_drag_constant, delta_time);
        }
        self.speed = self.speed.normalize_or_zero()
            * util::drag(self.speed.length(), self.drag_constant, delta_time);

        if *input_handler.left() {
            self.rotation_speed -= self.rotation_acceleration * delta_time;
        }
        if *input_handler.right() {
            self.rotation_speed += self.rotation_acceleration * delta_time;
        }
        self.rotation_speed =
            util::drag(self.rotation_speed, self.rotation_drag_constant, delta_time);

        self.x += self.speed.x * delta_time;
        self.y += self.speed.y * delta_time;
        self.rotation += self.rotation_speed * delta_time;

        // projectiles
        if *input_handler.shoot() && self.fire_ready {
            self.fire_ready = false;
            self.fire_timer.start_only_if_not_running();

            Ok(GameEvent::PlayerShoot(Box::new(BasicLaser::new(
                texture_handler,
                self.x,
                self.y,
                self.rotation,
            ))))
        } else {
            Ok(GameEvent::None)
        }
    }
}

impl<'texture_handler> Drawable<'texture_handler> for Player<'_, 'texture_handler> {
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
