use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[derive(derive_getters::Getters)]
pub struct InputHandler {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    shoot: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            shoot: false,
        }
    }

    pub fn consume(&mut self, event: &Event) -> bool {
        match event {
            Event::Quit { .. } => false,
            Event::KeyDown {
                keycode: Some(key_code),
                ..
            } => {
                match key_code {
                    Keycode::W => self.up = true,
                    Keycode::S => self.down = true,
                    Keycode::A => self.left = true,
                    Keycode::D => self.right = true,
                    Keycode::Space => self.shoot = true,
                    _ => (),
                };
                true
            }
            Event::KeyUp {
                keycode: Some(key_code),
                ..
            } => {
                match key_code {
                    Keycode::W => self.up = false,
                    Keycode::S => self.down = false,
                    Keycode::A => self.left = false,
                    Keycode::D => self.right = false,
                    Keycode::Space => self.shoot = false,
                    _ => (),
                };
                true
            }
            _ => true,
        }
    }
}
