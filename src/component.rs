use crate::{components::{enemy::Enemy, player::Player}, engine::Sprite, events::Event};

pub trait Component {
    fn update(&mut self, _: &Player, _: Vec<Enemy>) -> Option<Event> {
        None
    }

    fn visible(&self) -> bool {
        true
    }

    fn get_sprites(&self) -> Vec<Sprite>;
}
