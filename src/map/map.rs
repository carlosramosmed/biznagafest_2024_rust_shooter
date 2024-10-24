use MapObject::*;

use crate::{engine::TextureID};

use super::pos::Pos;

#[derive(Hash, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum MapObject {
    F, // floor

    #[default]
    W, // wall

    G, // Gargoils,
    B, // Bricks,
    S, // Shield,
    M, // Mold,
}

pub const WIDTH: usize = 16;
pub const HEIGHT: usize = 9;

pub static MAP: &'static [[MapObject; WIDTH]; HEIGHT] = &[
    [W, W, W, W, W, W, W, W, M, M, M, W, W, W, W, W],
    [W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W],
    [W, F, F, S, B, B, B, F, F, F, B, B, B, F, F, W],
    [W, F, F, F, F, F, B, F, F, F, F, F, B, F, F, W],
    [W, F, F, F, F, F, B, F, F, F, F, F, B, F, F, W],
    [W, F, F, S, B, B, B, F, F, F, F, F, F, F, F, W],
    [W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W],
    [W, F, F, M, F, F, F, M, F, F, F, F, F, F, F, W],
    [W, W, W, W, W, W, W, W, G, W, G, W, G, W, W, W],
];

pub fn is_wall(pos: Pos) -> bool {
    let map_pos = pos.map();

    is_in_map(map_pos.x(), map_pos.y()) && MAP[map_pos.y()][map_pos.x()] != MapObject::F
}

pub fn get_texture(pos: Pos) -> Option<MapObject> {
    let map_pos = pos.map();

    if is_in_map(map_pos.x(), map_pos.y()) {
        Some(MAP[map_pos.y()][map_pos.x()])
    } else {
        None
    }
}

pub fn is_in_map<T>(x: T, y: T) -> bool
where
    T: TryInto<usize>,
{
    if let (Ok(x_usize), Ok(y_usize)) = (x.try_into(), y.try_into()) {
        x_usize < WIDTH && y_usize < HEIGHT
    } else {
        false
    }
}

pub fn map_to_texture(map_object: MapObject) -> TextureID {
    match map_object {
        _ => TextureID::Wall,
    }
}