// TODO: ejercicio: añadir los módulos que faltan

mod events;
mod sequence;
mod soldier;
mod texture;

pub mod assets;
pub mod conf;

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
