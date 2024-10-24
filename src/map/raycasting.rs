use crate::conf::MAX_DEPTH;

use super::{
    map::{get_texture, is_wall, MapObject},
    pos::{MapPos, Pos},
    traverse::traverse_grid,
};

// Function to calculate horizontal intersections
pub fn calculate_horizontal_intersection(
    pos: &Pos,
    map_pos: &MapPos,
    sin_a: f32,
    cos_a: f32,
) -> (f32, f32, Option<MapObject>) {
    let (y_hor, dy) = if sin_a.is_sign_positive() {
        ((map_pos.y() + 1) as f32, 1.0)
    } else {
        (map_pos.y() as f32 - 1e-6, -1.0)
    };

    let depth_hor = (y_hor - pos.y()) / sin_a;
    let x_hor = pos.x() + depth_hor * cos_a;
    let delta_depth = dy / sin_a;
    let dx = delta_depth * cos_a;

    // Traverse the grid to find the wall
    let (depth_hor, x_hor, texture_hor) =
        traverse_grid(Pos::new(x_hor, y_hor), Pos::new(dx, dy), depth_hor, delta_depth);

    (depth_hor, x_hor, texture_hor)
}
// Function to calculate vertical intersections
pub fn calculate_vertical_intersection(
    pos: &Pos,
    map_pos: &MapPos,
    sin_a: f32,
    cos_a: f32,
) -> (f32, f32, Option<MapObject>) {
    let (mut x_vert, dx) = if cos_a.is_sign_positive() {
        ((map_pos.x() + 1) as f32, 1.0)
    } else {
        (map_pos.x() as f32 - 1e-6, -1.0)
    };

    let mut depth_vert = (x_vert - pos.x()) / cos_a;
    let mut y_vert = pos.y() + depth_vert * sin_a;
    let delta_depth = dx as f32 / cos_a;
    let dy = delta_depth * sin_a;
    let mut texture_vert = None;

    for _ in 1..MAX_DEPTH {
        if is_wall(Pos::new(x_vert, y_vert)) {
            texture_vert = get_texture(Pos::new(x_vert, y_vert));
            break;
        }
        x_vert += dx;
        y_vert += dy;
        depth_vert += delta_depth;
    }

    (depth_vert, y_vert, texture_vert)
}

// Function to select the closest intersection
pub fn select_closest_intersection(
    depth_hor: f32,
    x_hor: f32,
    texture_hor: Option<MapObject>,
    depth_vert: f32,
    y_vert: f32,
    texture_vert: Option<MapObject>,
    sin_a: f32,
    cos_a: f32,
) -> (f32, f32, Option<MapObject>) {
    if depth_vert < depth_hor {
        let offset = {
            let y_mod = y_vert % 1.0;
            if cos_a.is_sign_positive() {
                y_mod
            } else {
                1.0 - y_mod
            }
        };
        (depth_vert, offset, texture_vert)
    } else {
        let offset = {
            let x_mod = x_hor % 1.0;
            if sin_a.is_sign_positive() {
                1.0 - x_mod
            } else {
                x_mod
            }
        };
        (depth_hor, offset, texture_hor)
    }
}

// Function to correct the fishbowl effect
pub fn correct_fishbowl_effect(depth: f32, player_angle: f32, ray_angle: f32) -> f32 {
    depth * (player_angle - ray_angle).cos()
}

// Function to calculate the projected height of the wall slice
pub fn calculate_projection_height(screen_dist: f32, depth: f32) -> f32 {
    screen_dist / (depth + 0.0001)
}
