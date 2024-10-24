use std::f32::consts::TAU;

use crate::{
    component::Component,
    conf::{
        DELTA_ANGLE, HALF_FOV, HALF_WIDTH, NUM_RAYS, 
        PLAYER_ANGLE, PLAYER_POS, PLAYER_SPEED,
    },
    engine::Sprite,
    events::Event, map::{map::{is_wall, map_to_texture}, mov::Mov, pos::Pos, raycasting::{calculate_horizontal_intersection, calculate_projection_height, calculate_vertical_intersection, correct_fishbowl_effect, select_closest_intersection}},
};

use super::enemy::Enemy;

#[derive(Clone, Copy)]
pub struct Player {
    life: u32,
    pos: Pos,
    angle: f32,
    pain: bool,
    pain_count: u8,
}

impl Player {
    pub fn new_player() -> Player {
        return Player {
            life: 100,
            pos: Pos::new(PLAYER_POS.0, PLAYER_POS.1),
            angle: PLAYER_ANGLE,
            pain: false,
            pain_count: 100,
        };
    }

    pub fn hit(&mut self, damage: u32) {
        self.pain = true;
        self.life -= damage;
    }

    // TODO: ejercicio: escribir estos 3 metodos
    pub fn alive(&self) -> bool {

    }

    pub fn pos(&self) -> Pos {

    }

    pub fn angle(&self) -> f32 {

    }
    //

    pub fn spin(&mut self, angle: f32) {
        if angle.is_sign_positive() {
            if self.angle + angle > TAU {
                let dif = TAU - self.angle;
                self.angle = angle - dif;
                return;
            }
        } else {
            if self.angle + angle < 0.0 {
                let dif = self.angle + angle;
                self.angle = TAU + dif;
                return;
            }
        }

        self.angle += angle;
    }

    pub fn walk(&mut self, mov: Mov, delta_time: f32) {
        let (sin_a, cos_a) = (self.angle.sin(), self.angle.cos());

        let speed = PLAYER_SPEED * delta_time;

        let speed_sin = speed * sin_a;
        let speed_cos = speed * cos_a;

        let (dx, dy) = match mov {
            Mov::North => (speed_cos, speed_sin),
            Mov::South => (-speed_cos, -speed_sin),
            Mov::West => (speed_sin, -speed_cos),
            Mov::East => (-speed_sin, speed_cos),
        };

        let mov = self.pos + Pos::new(dx, dy);

        if !is_wall(mov) {
            self.pos = mov;
        }
    }

    fn viewport(&self) -> Vec<Sprite> {
        let mut ray_angle = self.angle - HALF_FOV + 0.0001;
        let mut i: usize = 1;

        let mut rays = Vec::with_capacity(NUM_RAYS);

        while i < NUM_RAYS {
            let ray = self.new_ray(i, ray_angle);
            ray_angle += DELTA_ANGLE;
            i += 1;
            rays.push(ray);
        }

        rays
    }

    fn new_ray(&self, num: usize, ray_angle: f32) -> Sprite {
        let map_pos = self.pos.map();
        let screen_dist = HALF_WIDTH as f32 / HALF_FOV.tan();
        let sin_a = ray_angle.sin();
        let cos_a = ray_angle.cos();
    
        // Calculate horizontal and vertical intersections
        let (depth_hor, x_hor, texture_hor) =
            calculate_horizontal_intersection(&self.pos, &map_pos, sin_a, cos_a);
        let (depth_vert, y_vert, texture_vert) =
            calculate_vertical_intersection(&self.pos, &map_pos, sin_a, cos_a);
    
        // Select the closest intersection
        let (mut depth, offset, texture) = select_closest_intersection(
            depth_hor,
            x_hor,
            texture_hor,
            depth_vert,
            y_vert,
            texture_vert,
            sin_a,
            cos_a,
        );
    
        // Correct for the fishbowl effect and project the wall slice height
        depth = correct_fishbowl_effect(depth, self.angle, ray_angle);
        let proj_height = calculate_projection_height(screen_dist, depth);
    
        Sprite::Column(
            depth,
            map_to_texture(texture.unwrap()),
            num,
            offset,
            proj_height,
        )
    }
}

impl Component for Player {
    fn update(&mut self, _: &Player, _: Vec<Enemy>) -> Option<Event> {
        if self.pain {
            self.pain_count += 1;

            if self.pain_count == 100 {
                self.pain = false;
                self.pain_count = 0;
            }
        }

        None
    }

    fn get_sprites(&self) -> Vec<Sprite> {
        let mut objects = self.viewport();

        objects.push(Sprite::LifeCounter(self.life));

        self.pain.then(|| objects.push(Sprite::PainScreen));

        objects
    }
}