use crate::game_object::Drawable;

pub enum GameEvent<'texture_handler> {
    None,
    TimerFire,
    PlayerShoot(Box<dyn Drawable<'texture_handler> + 'texture_handler>),
}
