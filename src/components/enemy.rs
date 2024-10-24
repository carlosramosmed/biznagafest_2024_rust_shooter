use std::f32::consts::{PI, TAU};

use crate::{
    component::Component, components::player::Player, conf::{
        screen_dist, DELTA_ANGLE, HALF_HEIGHT, HALF_NUM_RAYS, HALF_WIDTH, MAX_DEPTH, SCALE, WIDTH,
    }, engine::Sprite, events::Event, map::{map::is_wall, path::{PathFinding, WAYS}, pos::{MapPos, Pos}}, sequence::EnemySequence, texture::TextureRef
};

const HIT_MARGIN: f32 = 80.0;

type Projection = ((f32, f32), f32, f32);

#[derive(Clone)]
pub struct Enemy {
    life: i32,
    damage: u32,
    pos: Pos,

    pathfinding: PathFinding,

    sequence: EnemySequence,

    shift: f32,
    scale: f32,

    dying: bool,
    in_pain: bool,
    moving: bool,
    shooting: bool,

    shoot_wait: u32,

    screen_x: f32,
    dist: f32,
}

impl Component for Enemy {
    // could be in view but not visible, as another sprite or wall could be occluding it
    fn visible(&self) -> bool {
        let texture = self.get_texture();

        let image_half_width = (texture.width() / 2) as f32;

        -image_half_width < self.screen_x
            && self.screen_x < (WIDTH as f32 + image_half_width)
            && self.dist > 0.5
    }

    fn get_sprites(&self) -> Vec<Sprite> {
        let projection = self.get_projection();

        let render = Sprite::Sprite(
            self.dist,
            self.get_texture().id(),
            projection.0,
            projection.1,
            projection.2,
        );

        vec![render]
    }

    fn update(&mut self, player: &Player, enemies: Vec<Enemy>) -> Option<Event> {
        if self.in_pain {
            if self.sequence.pain_textures.done() {
                self.in_pain = false;
                self.sequence.pain_textures.reset();
            } else {
                self.sequence.pain_textures.next();
            }
        }

        if self.shoot_wait == 300 {
            self.shoot_wait = 0;
            self.shooting = false;
        }

        if self.shooting {
            self.shoot_wait += 1;
            self.sequence.shooting_textures.next();
        }

        if self.moving {
            self.sequence.walking_textures.next();
        }

        if self.life <= 0 {
            self.sequence.dying_textures.next();

            if self.sequence.dying_textures.last_frame() {
                self.sequence.dying_textures.block();
            }
        }

        self.update_projection(player);

        if self.alive() {
            let others = enemies
                .into_iter()
                .filter(|enemy| enemy.pos != self.pos)
                .collect();

            if !self.shooting{
                return self.movement(player, others);
            } else {
                return None;
            }
        }

        None
    }
}

impl Enemy {
    pub fn new(
        life: i32,
        damage: u32,
        pos: (f32, f32),
        sequences: EnemySequence,
        shift: f32,
        scale: f32,
    ) -> Self {
        Enemy {
            life,
            damage,
            pos: Pos::new(pos.0, pos.1),
            sequence: sequences,
            dying: false,
            shift,
            scale,

            pathfinding: PathFinding::new(),

            in_pain: false,
            moving: false,
            shooting: false,

            shoot_wait: 0,

            screen_x: 0.0,
            dist: 0.0,
        }
    }

    pub fn check_dead(&mut self) {
        if self.life <= 0 && !self.dying {
            self.dying = true;
        }
    }

    pub fn alive(&self) -> bool {
        self.life > 0
    }

    fn attack(&self) -> Event {
        Event::EnemyAttack(self.damage)
    }

    pub fn movement(&mut self, player: &Player, enemies: Vec<Enemy>) -> Option<Event> {
        if self.near_player(player) && !self.shooting {
            self.shooting = true;
            return Some(self.attack());
        }

        let enemies_pos = enemies
            .iter()
            .map(|enemy| (enemy.pos.map()))
            .map(MapPos::into)
            .collect();

        let maybe_path = self.pathfinding.get_path(
            self.pos.map().into(),
            player.pos().map().into(),
            enemies_pos,
        );

        match maybe_path {
            None => self.moving = false,
            Some(path) => {
                self.moving = true;

                let (next_x, next_y) = (path.x(), path.y());

                let angle =
                    (next_y as f32 + 0.5 - self.pos.y()).atan2(next_x as f32 + 0.5 - self.pos.x());
                let dx = angle.cos() * 0.002;
                let dy = angle.sin() * 0.002;

                self.pos = self.pos + Pos::new(dx, dy);
            }
        }

        None
    }

    fn get_texture(&self) -> TextureRef {
        if self.life <= 0 {
            self.sequence.dying_textures.get_texture()
        } else if self.shooting {
            self.sequence.shooting_textures.get_texture()
        } else if self.in_pain {
            self.sequence.pain_textures.get_texture()
        } else {
            self.sequence.walking_textures.get_texture()
        }
    }

    pub fn hit(&mut self, player: &Player) -> bool {
        let map_pos = (self.pos.x() as u32, self.pos.y() as u32);

        let d = self.pos - player.pos();

        let theta = d.y().atan2(d.x());

        let in_screen = HALF_WIDTH as f32 - HIT_MARGIN < self.screen_x
            && self.screen_x < HALF_WIDTH as f32 + HIT_MARGIN;

        let not_wall_between = self.new_ray(player.pos(), theta, map_pos, player.pos().into());

        if in_screen && not_wall_between {
            self.in_pain = true;

            return true;
        } else {
            false
        }
    }

    pub fn receive_damage(&mut self, damage: i32) {
        self.life -= damage;

        self.check_dead();
    }

    fn new_ray(
        &self,
        o: Pos,
        ray_angle: f32,
        (x_map, y_map): (u32, u32),
        (player_x_map, player_y_map): (u32, u32),
    ) -> bool {
        if x_map == player_x_map && y_map == player_y_map {
            return true;
        }

        let (mut wall_dist_v, mut wall_dist_h) = (0.0, 0.0);
        let (mut player_dist_v, mut player_dist_h) = (0.0, 0.0);

        let (sin_a, cos_a) = (ray_angle.sin(), ray_angle.cos());

        // horizontals
        let (mut y_hor, dy) = if sin_a.is_sign_positive() {
            ((player_y_map + 1) as f32, 1.0)
        } else {
            (player_y_map as f32 - 1e-6, -1.0)
        };

        let mut depth_hor = (y_hor - o.y()) / sin_a;
        let mut x_hor = o.x() + depth_hor * cos_a;

        let delta_depth = dy / sin_a;
        let dx = delta_depth * cos_a;

        for _ in 0..MAX_DEPTH {
            if x_hor as u32 == x_map && y_hor as u32 == y_map {
                player_dist_h = depth_hor;
                break;
            }

            if is_wall(Pos::new(x_hor, y_hor)) {
                wall_dist_h = depth_hor;
                break;
            }

            x_hor += dx;
            y_hor += dy;

            depth_hor += delta_depth;
        }

        // verticals
        let (mut x_vert, dx) = if cos_a.is_sign_positive() {
            ((player_x_map + 1) as f32, 1.0)
        } else {
            (player_x_map as f32 - 1e-6, -1.0)
        };

        let mut depth_vert = (x_vert - o.x()) / cos_a;
        let mut y_vert = o.y() + depth_vert * sin_a;

        let delta_depth = dx as f32 / cos_a;
        let dy = delta_depth * sin_a;

        for _ in 0..MAX_DEPTH {
            if x_vert as u32 == x_map && y_vert as u32 == y_map {
                player_dist_v = depth_vert;
                break;
            }

            if is_wall(Pos::new(x_vert, y_vert)) {
                wall_dist_v = depth_vert;
                break;
            }

            x_vert += dx;
            y_vert += dy;

            depth_vert += delta_depth;
        }

        let player_dist = if player_dist_v > player_dist_h {
            player_dist_v
        } else {
            player_dist_h
        };
        let wall_dist = if wall_dist_v > wall_dist_h {
            wall_dist_v
        } else {
            wall_dist_h
        };

        (0.0 < player_dist && player_dist < wall_dist) || wall_dist == 0.0
    }

    fn near_player(&self, player: &Player) -> bool {
        let mut result = false;

        WAYS.into_iter().for_each(|way| {
            if (self.pos + way.into()).map() == player.pos().map() {
                result = true;
            }
        });

        result
    }

    fn update_projection(&mut self, player: &Player) {
        let d = self.pos - player.pos();

        let theta = d.y().atan2(d.x());
        let mut delta = theta - player.angle();

        if (d.x() > 0.0 && player.angle() > PI) || (d.x() < 0.0 && d.y() < 0.0) {
            delta += TAU;
        }

        let delta_rays = delta / DELTA_ANGLE;

        self.screen_x = (HALF_NUM_RAYS as f32 + delta_rays) * SCALE as f32;

        self.dist = d.y().hypot(d.x()) * delta.cos(); // normalized
    }

    fn get_projection(&self) -> Projection {
        let proj_height = screen_dist() / self.dist * self.scale;
        let proj_width = proj_height * self.get_texture().ratio();
        let height_shift = proj_height * self.shift;

        let sprite_half_width = proj_width / 2.0;

        let location = (
            self.screen_x - sprite_half_width,
            HALF_HEIGHT as f32 - proj_height / 2.0 + height_shift,
        );

        (location, proj_width, proj_height)
    }
}
