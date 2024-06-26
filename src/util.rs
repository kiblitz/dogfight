use std::time::Duration;

use sdl2::rect::Rect;

const NS_IN_S: u32 = 1_000_000_000u32;
const REFRESH_RATE: u32 = 60;
pub const REFRESH_EVERY: Duration = Duration::new(0, NS_IN_S / REFRESH_RATE);

#[warn(dead_code)]
#[inline(always)]
pub fn lerp(current: f32, target: f32, c: f32, delta_time: f32) -> f32 {
    let percent = 1. - c.powf(delta_time);
    return current + (target - current) * percent;
}

#[inline(always)]
pub fn drag(current: f32, c: f32, delta_time: f32) -> f32 {
    return current * c.powf(delta_time);
}

pub fn rect(x: f32, y: f32, width: f32, height: f32) -> Rect {
    Rect::new(
        (x - width / 2.) as i32,
        (y - height / 2.) as i32,
        width as u32,
        height as u32,
    )
}
