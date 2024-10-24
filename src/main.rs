use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

use doomie::assets::TEXTURES_PATHS;
use doomie::conf::{FPS, HEIGHT, WIDTH};
use doomie::sdl::{SDLengine, ScaledTexture};
use doomie::game::Game;
use sdl2::image::LoadTexture;

pub fn main() -> Result<(), Box<dyn Error>> {
    let sdl = sdl2::init()?;

    let (clock, events, video) = (sdl.timer()?, sdl.event_pump()?, sdl.video()?);
    let window = video
        .window("doomie", WIDTH, HEIGHT)
        .position_centered()
        .build()?;
    let screen = window.into_canvas().build()?;
    let texture_creator = screen.texture_creator();

    let mut textures = HashMap::new();

    TEXTURES_PATHS.iter().for_each(|(texture_id, path)| {
        let texture = texture_creator.load_texture(Path::new(path)).unwrap();

        textures.insert(*texture_id, ScaledTexture::new(texture));
    });

    let engine = SDLengine::new(sdl.mouse(), screen, textures, events, clock, FPS);

    let mut game = Game::new(engine);

    game.run();

    Ok(())
}