use crate::camera::Camera;
use crate::game_event::GameEvent;
use crate::game_object::Updatable;
use crate::texture_handler::TextureHandler;

pub struct TimerObject {
    length: f32,
    status: Status,
}

enum Status {
    NotRunning,
    Running(f32),
}

impl TimerObject {
    pub fn new(length: f32) -> Self {
        Self {
            length,
            status: Status::NotRunning,
        }
    }

    pub fn start_only_if_not_running(&mut self) {
        match self.status {
            Status::NotRunning => {
                self.status = Status::Running(self.length);
            }
            _ => (),
        };
    }
}

impl<'texture_handler> Updatable<'texture_handler> for TimerObject {
    fn update(
        &mut self,
        _: &mut Camera,
        _: &'texture_handler TextureHandler,
        _: &crate::input_handler::InputHandler,
        delta_time: f32,
    ) -> Result<GameEvent<'texture_handler>, Box<dyn std::error::Error>> {
        Ok(match self.status {
            Status::NotRunning => GameEvent::None,
            Status::Running(left) => {
                let new_left = left - delta_time;
                if new_left <= 0. {
                    self.status = Status::NotRunning;
                    GameEvent::TimerFire
                } else {
                    self.status = Status::Running(new_left);
                    GameEvent::None
                }
            }
        })
    }
}
