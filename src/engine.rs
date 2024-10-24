use std::f32::{INFINITY, NEG_INFINITY};

use crate::texture::TextureRef;

pub trait Driver {
    fn delta_time(&mut self) -> f32;

    // controls
    fn poll(&mut self) -> Option<Controlls>;

    // audio
    fn play_shoot(&mut self);
    fn play_pain_shout(&mut self);
    fn play_enemy_shoot(&mut self);
    fn play_player_pain(&mut self);

    // rendering
    fn render(&mut self, commands: Vec<Sprite>);
    fn get_texture_width(&self, texture_id: TextureID) -> u32;
    fn get_image_ratio(&self, texture_id: TextureID) -> f32;
    fn center_mouse(&self, x: i32, y: i32);
    fn load_refs(&self, ids: &[TextureID]) -> Vec<TextureRef>;
}

#[derive(Clone, Copy)]
pub enum Sprite {
    Background,
    PainScreen,
    GameOver,
    LifeCounter(u32),
    DrawWeapon(TextureID),
    Column(f32, TextureID, usize, f32, f32),
    Sprite(f32, TextureID, (f32, f32), f32, f32),
}

impl Sprite {
    pub fn z_order(&self) -> f32 {
        match self {
            Sprite::Column(z_order, ..) => *z_order,
            Sprite::Sprite(z_order, ..) => *z_order,
            Sprite::Background          => INFINITY,
            Sprite::DrawWeapon(_)       => NEG_INFINITY,
            Sprite::PainScreen          => NEG_INFINITY,
            Sprite::LifeCounter(_)      => NEG_INFINITY,
            Sprite::GameOver            => NEG_INFINITY,
        }
    }
}

// Include the generated enum definition
include!(concat!(env!("OUT_DIR"), "/texture_id.rs"));

/* 
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TextureID {
    Wall,

    WeaponIdle,
    WeaponShoot,
    WeaponReload,
    WeaponReload2,
    WeaponReload3,
    WeaponReload4,

    SoldierAlive,
    SoldierAlive2,
    SoldierAlive3,
    SoldierAlive4,
    SoldierAlive5,
    SoldierAlive6,
    SoldierAlive7,
    SoldierAlive8,
    SoldierPain,
    SoldierDead,
    SoldierDead2,
    SoldierDead3,
    SoldierDead4,
    SoldierDead5,
    SoldierDead6,
    SoldierDead7,
    SoldierDead8,

    SoldierWalking,
    SoldierWalking2,
    SoldierWalking3,
    SoldierWalking4,

    SoldierShooting,
    SoldierShooting1,

    PainScreen,
    GameOver,

    Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine,
}
*/

pub fn digit_to_texture(c: char) -> TextureID {
    match c {
        '0' => TextureID::Zero,
        '1' => TextureID::One,
        '2' => TextureID::Two,
        '3' => TextureID::Three,
        '4' => TextureID::Four,
        '5' => TextureID::Five,
        '6' => TextureID::Six,
        '7' => TextureID::Seven,
        '8' => TextureID::Eight,
        '9' => TextureID::Nine,
        _   => TextureID::Zero
    }
}

#[derive(Clone, Copy)]
pub enum Controlls {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,

    Enter,
    Escape,

    Spin(i32, i32), // (x, xrel)
}
