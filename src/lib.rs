mod component;
mod engine;
mod events;
mod sequence;
mod soldier;
mod texture;

// deben ser publicos los que se usan en el main
pub mod assets;
pub mod conf;
pub mod game;

pub mod sdl;

mod map {
    pub mod map;
    pub mod mov;
    pub mod path;
    pub mod pos;
    pub mod raycasting;
    mod traverse;
}

mod components {
    pub mod background;
    pub mod enemy;
    pub mod player;
    pub mod weapon;
}
