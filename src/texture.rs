use crate::engine::TextureID;

#[derive(Copy, Clone)]
pub struct TextureRef {
    id: TextureID,
    width: u32,
    ratio: f32,
}

impl TextureRef {
    pub fn new(id: TextureID, width: u32, ratio: f32) -> TextureRef {
        TextureRef { id, width, ratio }
    }

    pub fn id(&self) -> TextureID {
        self.id
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn ratio(&self) -> f32 {
        self.ratio
    }
}