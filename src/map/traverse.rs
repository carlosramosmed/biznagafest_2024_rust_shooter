use crate::conf::MAX_DEPTH;

use super::{
    map::{get_texture, is_wall, MapObject},
    pos::Pos,
};

pub fn traverse_grid(
    mut pos: Pos,
    dist: Pos,
    mut depth: f32,
    delta_depth: f32,
) -> (f32, f32, Option<MapObject>) {
    let mut texture = None;

    for _ in 1::MAX_DEPTH {
        if is_wall(pos) {
            texture = get_texture(pos);
            break
        }
        pos = pos + dist
        depth += delta_depth
    }

    (depth, pos.x(), texture)
}
