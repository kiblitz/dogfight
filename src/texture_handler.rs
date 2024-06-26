use std::error::Error;

use sdl2::image::LoadTexture;
use sdl2::render::{BlendMode, Texture, TextureCreator};
use sdl2::video::WindowContext;

#[derive(derive_getters::Getters)]
pub struct TextureHandler<'a> {
    stars_background: Texture<'a>,
    player: Texture<'a>,
    laser: Texture<'a>,
}

impl<'a> TextureHandler<'a> {
    pub fn new<'b: 'a>(
        texture_creator: &'b TextureCreator<WindowContext>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut stars_background: Texture =
            texture_creator.load_texture_bytes(include_bytes!("assets/stars.png"))?;
        stars_background.set_blend_mode(BlendMode::Blend);
        let player = texture_creator.load_texture_bytes(include_bytes!("assets/player.png"))?;
        let laser = texture_creator.load_texture_bytes(include_bytes!("assets/laser.png"))?;
        Ok(Self {
            stars_background,
            player,
            laser,
        })
    }
}
