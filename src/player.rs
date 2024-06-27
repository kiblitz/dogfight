use crate::basic_laser::BasicLaser;
use crate::game_event::GameEvent;
use crate::game_object::{Bounds, Drawable, GameTexture, Updatable};
use crate::input_handler::InputHandler;
use crate::texture_handler::TextureHandler;
use crate::timer_object::TimerObject;
use crate::util;

use std::error::Error;
use std::f32::consts::PI;
use std::time::Duration;

use glam::Vec2;

use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(derive_getters::Getters)]
pub struct Player<'player, 'texture_handler> {
    bounds: Bounds,
    physics: Physics,
    rotation_physics: RotationPhysics,
    game_texture: GameTexture<'player, 'texture_handler>,
    fire_timer: TimerObject,
    fire_ready: bool,
}

struct Physics {
    speed: Vec2,
    acceleration: f32,
    drag_constant: f32,
    break_drag_constant: f32,
}

struct RotationPhysics {
    value: f32,
    speed: f32,
    acceleration: f32,
    drag_constant: f32,
}

impl<'texture_handler> Player<'_, 'texture_handler> {
    pub fn new(texture_handler: &'texture_handler TextureHandler) -> Self {
        Self {
            bounds: Bounds {
                x: 0.,
                y: 0.,
                width: 100.,
                height: 100.,
            },
            physics: Physics {
                speed: Vec2::ZERO,
                acceleration: 420.,
                drag_constant: 0.6,
                break_drag_constant: 0.3,
            },
            rotation_physics: RotationPhysics {
                value: 0.,
                speed: 0.,
                acceleration: 12.,
                drag_constant: 0.15,
            },
            game_texture: GameTexture {
                texture: texture_handler.player(),
                rotation_offset: PI / 2.,
            },
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
            self.physics.speed += Vec2::from_angle(self.rotation_physics.value)
                * self.physics.acceleration
                * delta_time;
        } else if *input_handler.down() && !input_handler.up() {
            self.physics.speed = self.physics.speed.normalize_or_zero()
                * util::drag(
                    self.physics.speed.length(),
                    self.physics.break_drag_constant,
                    delta_time,
                );
        }
        self.physics.speed = self.physics.speed.normalize_or_zero()
            * util::drag(
                self.physics.speed.length(),
                self.physics.drag_constant,
                delta_time,
            );

        if *input_handler.left() {
            self.rotation_physics.speed -= self.rotation_physics.acceleration * delta_time;
        }
        if *input_handler.right() {
            self.rotation_physics.speed += self.rotation_physics.acceleration * delta_time;
        }
        self.rotation_physics.speed = util::drag(
            self.rotation_physics.speed,
            self.rotation_physics.drag_constant,
            delta_time,
        );

        self.bounds.x += self.physics.speed.x * delta_time;
        self.bounds.y += self.physics.speed.y * delta_time;
        self.rotation_physics.value += self.rotation_physics.speed * delta_time;

        // projectiles
        if *input_handler.shoot() && self.fire_ready {
            self.fire_ready = false;
            self.fire_timer.start_only_if_not_running();

            Ok(GameEvent::PlayerShoot(Box::new(BasicLaser::new(
                texture_handler,
                self.bounds.x,
                self.bounds.y,
                self.rotation_physics.value,
            ))))
        } else {
            Ok(GameEvent::None)
        }
    }
}

impl<'texture_handler> Drawable<'texture_handler> for Player<'_, 'texture_handler> {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>> {
        canvas.copy_ex(
            self.game_texture.texture,
            None,
            Some(util::rect(
                self.bounds.x,
                self.bounds.y,
                self.bounds.width,
                self.bounds.height,
            )),
            (self.rotation_physics.value + self.game_texture.rotation_offset).to_degrees() as f64,
            None,
            false,
            false,
        )?;
        Ok(())
    }
}
