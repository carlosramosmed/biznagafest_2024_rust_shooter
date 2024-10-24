

use std::f32::consts::PI;

pub const WIDTH:u32 = 1600;
pub const HEIGHT:u32 = 900;

pub const HALF_WIDTH:u32 = WIDTH / 2;
pub const HALF_HEIGHT:u32 = HEIGHT / 2;

pub const FPS:u32 = 60;

pub const PLAYER_POS:(f32, f32) = (1.5, 5.0);
pub const PLAYER_SPEED:f32 = 0.004;
pub const PLAYER_ANGLE:f32 = 0.0;
pub const PLAYER_ROT_SPEED:f32 = 0.002;

// raycasting
pub const FOV: f32 = PI / 2.0;
pub const HALF_FOV: f32 = FOV / 2.0;
pub const NUM_RAYS: usize = WIDTH as usize / 2;
pub const HALF_NUM_RAYS: usize = NUM_RAYS / 2;
pub const DELTA_ANGLE: f32 = FOV / NUM_RAYS as f32;
pub const MAX_DEPTH: u32 = 20;

pub const SCALE: u32 = WIDTH / NUM_RAYS as u32;

pub const TEXTURE_SIZE: u32 = 256;
pub const HALF_TEXTURE_SIZE: u32 = TEXTURE_SIZE / 2;

pub const MOUSE_SENSITIVITY: f32 = 0.0003;
pub const MOUSE_MAX_REL: i32 = 40;
pub const MOUSE_BORDER_LEFT: i32 = 100;
pub const MOUSE_BORDER_RIGHT: i32 = (WIDTH as i32) - MOUSE_BORDER_LEFT;

pub const FLOOR_COLORS: (u32, u32, u32) = (30, 30, 30);

pub fn screen_dist() -> f32 {
    HALF_WIDTH as f32 / HALF_FOV.tan()
}