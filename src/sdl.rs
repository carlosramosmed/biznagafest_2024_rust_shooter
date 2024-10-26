use std::collections::HashMap;

use sdl2::{event::Event, keyboard::Keycode, mixer::{self, Channel, Chunk, InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS}, mouse::MouseUtil, pixels::Color, rect::Rect, render::{Texture, WindowCanvas}, EventPump, TimerSubsystem};

use crate::{conf::{HALF_HEIGHT, HALF_WIDTH, HEIGHT, SCALE, TEXTURE_SIZE, WIDTH}, engine::{digit_to_texture, Controlls, Driver, Sprite, TextureID}, texture::TextureRef};

pub struct ScaledTexture<'a> {
    pub texture: Texture<'a>,
    pub scale_factor: f32,
}

impl <'a>ScaledTexture<'a> {
    pub fn new(texture: Texture<'a>) -> ScaledTexture {
        ScaledTexture {
            texture,
            scale_factor: 4.0,
        }
    }
}

// <'a> -> tiempo de vida del objeto, lo necesita el compilador (?)
pub struct SDLengine<'a> {
    screen: WindowCanvas,
    events: EventPump,
    clock: TimerSubsystem,

    shoot: Chunk,
    enemy_pain_shout: Chunk,
    enemy_shoot: Chunk,
    player_pain: Chunk,
    
    textures: HashMap<TextureID, ScaledTexture<'a>>,
    fps: u32,
    mouse: MouseUtil,
}

impl <'a> SDLengine <'a> {
    pub fn new(    
        mouse: MouseUtil,
        screen: WindowCanvas,
        textures: HashMap<TextureID, ScaledTexture<'a>>,
        events: EventPump, 
        clock: TimerSubsystem, 
        fps: u32
    ) -> Box<Self> {
        mouse.capture(false);

        // Initialize the SDL2_mixer with the desired format
        mixer::open_audio(44100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024).unwrap();
        mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG).unwrap();
        mixer::allocate_channels(32); // Allocate more channels to prevent running out

        let shoot = Chunk::from_file("assets/sound/shotgun.wav").unwrap();
        let enemy_pain_shout = Chunk::from_file("assets/sound/npc_pain.wav").unwrap();
        let enemy_shoot = Chunk::from_file("assets/sound/npc_attack.wav").unwrap();
        let player_pain = Chunk::from_file("assets/sound/player_pain.wav").unwrap();

        Box::new(Self {
            events, clock, screen, fps, textures, mouse,
            shoot, enemy_pain_shout, enemy_shoot, player_pain,
        })
    }

    fn render_texture(&mut self, texture_id: TextureID, pos: (f32, f32), proj_width: f32, proj_height: f32) {
        let texture  = self.textures.get(&texture_id).unwrap();

        let location = Rect::new(pos.0 as i32, pos.1 as i32, proj_width as u32, proj_height as u32);

        self.screen.copy(&texture.texture, None, location).unwrap();
    }

    fn render_background(&mut self) {
        let black_half = Rect::new(0, 0,                  WIDTH as u32, HALF_HEIGHT as u32);
        let gray_half  = Rect::new(0, HALF_HEIGHT as i32, WIDTH as u32, HALF_HEIGHT as u32);

        // Fill the rectangles with black and gray colors
        self.screen.set_draw_color(Color::RGB(0, 0, 0));
        self.screen.fill_rect(black_half).unwrap();
        self.screen.set_draw_color(Color::RGB(128, 128, 128));
        self.screen.fill_rect(gray_half).unwrap();
    }

    fn render_column(&mut self, texture_id: &TextureID, num: usize, offset: f32, proj_height: f32) {
        let texture  = self.textures.get(&texture_id).unwrap();

        let origin   = (offset * (TEXTURE_SIZE - SCALE) as f32 * texture.scale_factor) as i32;

        let portion  = Rect::new(origin, 0, SCALE, TEXTURE_SIZE * 15);
        let location = Rect::new((num * (SCALE as usize)) as i32,
                                        HALF_HEIGHT   as i32 - (proj_height / 2.0) as i32, 
                                        SCALE         as u32, 
                                        proj_height   as u32);

        self.screen.copy(&texture.texture, portion, location).unwrap();
    }
}

impl <'a> Driver for SDLengine<'a> {
    // fn delta_time(&mut self) -> f32 {
    //     self.clock.delay(1000 / self.fps);
    //     self.clock.ticks() as f32 / 1000.0
    // }

    fn delta_time(&mut self) -> f32 {
        let start = self.clock.ticks();

        self.clock.delay(self.fps);

        let end = self.clock.ticks();

        let delta = end - start;

        delta as f32
    }

    // Reserve channel 0 for shoot sounds
    fn play_shoot(&mut self) {
        if let Err(e) = Channel(0).play(&self.shoot, 0) {
            eprintln!("Failed to play shoot sound on channel 0: {}", e);
        }
    }

    fn play_enemy_shoot(&mut self) {
        if let Err(e) = Channel(1).play(&self.enemy_shoot, 0) {
            eprintln!("Failed to play shoot sound on channel 0: {}", e);
        }
    }

    // Use other channels for pain shouts
    fn play_pain_shout(&mut self) {
        if let Err(e) = Channel(2).play(&self.enemy_pain_shout, 0) {
            eprintln!("Failed to play pain shout sound on channel 1: {}", e);
        }
    }

    // Use other channels for pain shouts
    fn play_player_pain(&mut self) {
        if let Err(e) = Channel(3).play(&self.player_pain, 0) {
            eprintln!("Failed to play pain shout sound on channel 1: {}", e);
        }
    }

    fn poll(&mut self) -> Option<Controlls> {
        match self.events.poll_iter().next() {
            Some(event) => {
                match event {
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        Some(Controlls::Escape)
                    },
                    Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                        Some(Controlls::ArrowUp)
                    },
                    Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                        Some(Controlls::ArrowDown)
                    },
                    Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                        Some(Controlls::ArrowLeft)
                    },
                    Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                        Some(Controlls::ArrowRight)
                    },
                    Event::MouseMotion { x, xrel, .. } => {
                        Some(Controlls::Spin(x, xrel))
                    }
                    Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                        Some(Controlls::Enter)
                    },
                    _ => None
                }
            },
            None => None,
        }
    }

    fn center_mouse(&self, x: i32, y: i32) {
        self.mouse.warp_mouse_in_window(&self.screen.window(), x, y);
    }

    fn get_texture_width(&self, texture_id: TextureID) -> u32 {
        self.textures.get(&texture_id).unwrap().texture.query().width
    }

    fn get_image_ratio(&self, texture_id: TextureID) -> f32 {
        let texture = self.textures.get(&texture_id).unwrap();

        let width  = texture.texture.query().width;  
        let height = texture.texture.query().height;

        width as f32 / height as f32
    }

    fn load_refs(&self, ids: &[TextureID]) -> Vec<TextureRef> {
        ids.iter().map(|id| {
            let width = self.get_texture_width(*id);
            let ratio = self.get_image_ratio(*id);

            TextureRef::new(*id, width, ratio)
        }).collect()
    }

    fn render(&mut self, mut commands: Vec<Sprite>) {
        commands.sort_by(|a, b| b.z_order().partial_cmp(&a.z_order()).unwrap());

        commands.into_iter().for_each(|command| {
            match command {
                Sprite::Background => self.render_background(),
                Sprite::Column(_, texture_id, num, offset, proj_height) => {
                    self.render_column(&texture_id, num, offset, proj_height);
                }
                Sprite::Sprite(_, texture_id, pos, proj_width, proj_height) => {
                    self.render_texture(texture_id, pos, proj_width, proj_height);
                }
                Sprite::DrawWeapon(texture_id) => {
                    let texture  = self.textures.get(&texture_id).unwrap();
                    let width    = texture.texture.query().width;
                    let height   = texture.texture.query().height;

                    let location = Rect::new(
                                    (HALF_WIDTH - (width / 4)) as i32, 
                                    HEIGHT as i32 - (height / 2) as i32,
                                    width / 2, 
                                    height / 2);

                    self.screen.copy(&texture.texture, None, location).unwrap();
                }
                Sprite::PainScreen => {
                    let texture = self.textures.get(&TextureID::PainScreen).unwrap();

                    let location = Rect::new(0, 0, WIDTH, HEIGHT);

                    self.screen.copy(&texture.texture, None, location).unwrap();
                }
                Sprite::LifeCounter(life) => {
                    let counter = life.to_string();

                    for (i, digit) in counter.char_indices() {
                        let texture_id = digit_to_texture(digit);

                        let texture = self.textures.get(&texture_id).unwrap();

                        let width = texture.texture.query().width;
                        let height = texture.texture.query().height;

                        let location = Rect::new((i as u32 * width) as i32, 0, width, height);

                        self.screen.copy(&texture.texture, None, location).unwrap();
                    }
                }
                Sprite::GameOver => {
                    let texture = self.textures.get(&TextureID::GameOver).unwrap();

                    let location = Rect::new(0, 0, WIDTH, HEIGHT);

                    self.screen.copy(&texture.texture, None, location).unwrap();
                }
            }
        });

        self.screen.present();
    }
}