use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    x: f32,
    y: f32,
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Pos { x, y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn map(&self) -> MapPos {
        MapPos {
            x: self.x as usize,
            y: self.y as usize,
        }
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Pos {}

impl Into<(u32, u32)> for Pos {
    fn into(self) -> (u32, u32) {
        (self.x as u32, self.y as u32)
    }
}

impl From<(i32, i32)> for Pos {
    fn from(pos: (i32, i32)) -> Self {
        Pos {
            x: pos.0 as f32,
            y: pos.1 as f32,
        }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash)]
pub struct MapPos {
    x: usize,
    y: usize,
}

impl MapPos {
    pub fn new(x: usize, y: usize) -> Self {
        MapPos { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

impl Into<(i32, i32)> for MapPos {
    fn into(self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }
}

impl PartialEq for MapPos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for MapPos {}