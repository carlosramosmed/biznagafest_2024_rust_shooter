use crate::{
    component::Component,
    conf::FPS,
    engine::{Sprite, TextureID},
    events::Event,
    sequence::Sequence,
    texture::TextureRef,
};

use super::{enemy::Enemy, player::Player};

#[derive(Clone)]
enum WeaponState {
    Idle, Shooting, Reloading
}

#[derive(Clone)]
pub struct Weapon {
    damage: i32,
    state: WeaponState,
    idle: TextureID,
    shoot: Sequence,
    reload: Sequence,
}

impl Weapon {
    pub fn new(shooting_vec: Vec<TextureRef>, reload_vec: Vec<TextureRef>) -> Self {
        Weapon {
            damage: 50,
            state: WeaponState::Idle,
            idle: TextureID::WeaponIdle,
            shoot: Sequence::new(shooting_vec, FPS + 30),
            reload: Sequence::new(reload_vec, FPS + 30),
        }
    }

    pub fn damage(&self) -> i32 {
        self.damage
    }

    pub fn shoot(&mut self) {
        self.state = WeaponState::Shooting;
    }
}

impl Component for Weapon {
    fn update(&mut self, _: &Player, _: Vec<Enemy>) -> Option<Event> {
        match self.state {
            WeaponState::Idle => None,
            WeaponState::Shooting => {
                if self.shoot.last_frame() {
                    self.state = WeaponState::Reloading;
                    self.shoot.reset();
                } else {
                    self.shoot.next();
                }

                None
            }
            WeaponState::Reloading => {
                if self.reload.last_frame() {
                    self.state = WeaponState::Idle;
                    self.reload.reset();
                } else {
                    self.reload.next();
                }

                None
            }
        }
    }

    fn get_sprites(&self) -> Vec<Sprite> {
        let object = match self.state {
            WeaponState::Idle      => self.idle,
            WeaponState::Shooting  => self.shoot.get_texture_id(),
            WeaponState::Reloading => self.reload.get_texture_id(),
        };

        vec![Sprite::DrawWeapon(object)]
    }
}

pub static SHOOTING_TEXTURES: [TextureID; 2] = [TextureID::WeaponShoot, TextureID::WeaponReload1];
pub static RELOAD_TEXTURES: [TextureID; 3] = [
    TextureID::WeaponReload2,
    TextureID::WeaponReload3,
    TextureID::WeaponReload4,
];