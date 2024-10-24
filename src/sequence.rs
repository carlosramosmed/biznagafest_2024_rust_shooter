use crate::{conf::FPS, engine::TextureID, texture::TextureRef};

#[derive(Clone)]
pub struct EnemySequence {
    pub walking_textures: Sequence,
    pub dying_textures: Sequence,
    pub shooting_textures: Sequence,
    pub pain_textures: Sequence,
}

impl EnemySequence {
    pub fn new(
        walking_textures: Vec<TextureRef>,
        shooting_textures: Vec<TextureRef>,
        pain_texture: Vec<TextureRef>,
        dying_textures: Vec<TextureRef>,
    ) -> Self {
        let walking_textures = Sequence::new(walking_textures, FPS * 2);
        let shooting_textures = Sequence::new(shooting_textures, FPS * 3);
        let pain_textures = Sequence::new(pain_texture, FPS * 3);
        let dying_textures = Sequence::new(dying_textures, FPS);

        EnemySequence {
            walking_textures,
            shooting_textures,
            pain_textures,
            dying_textures,
        }
    }
}

#[derive(Clone)]
pub struct Sequence {
    textures: Vec<TextureRef>,
    current_texture: usize,
    animation_count: u32,
    blocked: bool,

    count: u32,
}

impl Sequence {
    pub fn new(textures: Vec<TextureRef>, count: u32) -> Self {
        Sequence {
            textures,
            current_texture: 0,
            animation_count: 0,
            blocked: false,
            count,
        }
    }

    fn texture_timeout(&self) -> bool {
        self.animation_count >= self.count
    }

    pub fn done(&self) -> bool {
        self.last_frame() && self.texture_timeout()
    }

    pub fn get_texture(&self) -> TextureRef {
        self.textures[self.current_texture]
    }

    pub fn get_texture_id(&self) -> TextureID {
        self.textures[self.current_texture].id()
    }

    pub fn last_frame(&self) -> bool {
        self.current_texture == self.textures.len() - 1
    }

    pub fn reset(&mut self) {
        self.current_texture = 0;
        self.animation_count = 0;
        self.unblock();
    }

    pub fn next(&mut self) {
        if self.blocked {
            return;
        }

        self.animation_count += 1;

        if self.texture_timeout() {
            if self.textures.len() == 1 {
                return;
            }

            self.next_frame();
            self.animation_count = 0;
        }
    }

    pub fn block(&mut self) {
        self.blocked = true;
    }

    fn unblock(&mut self) {
        self.blocked = false;
    }

    fn next_frame(&mut self) {
        self.current_texture = (self.current_texture + 1) % self.textures.len();
    }
}