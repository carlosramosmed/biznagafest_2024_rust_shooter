use crate::conf::MAX_DEPTH;

use super::{
    map::{get_texture, is_wall, MapObject},
    pos::Pos,
};

// TODO: ejercicio: completar el cuerpo de la función
pub fn traverse_grid(
    mut pos: Pos,
    dist: Pos,
    mut depth: f32,
    delta_depth: f32,
) -> (f32, f32, Option<MapObject>) {
    let mut texture = None;

    // ...

    (depth, pos.x(), texture)
}
