use std::{
    cmp::{max, min},
    ops::Not,
};

use crate::{
    component::Component, components::{
        background::Background, enemy::Enemy, player::Player, weapon::{Weapon, RELOAD_TEXTURES, SHOOTING_TEXTURES}
    }, conf::{
        HALF_HEIGHT, HALF_WIDTH, MOUSE_BORDER_LEFT, MOUSE_BORDER_RIGHT, MOUSE_MAX_REL,
        MOUSE_SENSITIVITY,
    }, engine::{Controlls, Driver, Sprite}, events::Event, map::mov::Mov, sequence::EnemySequence, soldier::{SOLDIER_DYING, SOLDIER_PAIN, SOLDIER_SHOTING, SOLDIER_WALKING}
};

const DELTA_TIME: f32 = 60.0;

pub struct Game<'a> {
    engine: Box<dyn Driver + 'a>,

    background: Background,
    player: Player,
    enemies: Vec<Enemy>,
    weapon: Weapon,
    is_over: bool,
}

impl<'a> Game<'a> {
    pub fn new( engine: Box<dyn Driver + 'a>) -> Box<Self> {
        let background = Background {};
        
        let player = Player::new_player();

        let weapon = Weapon::new(
            engine.load_refs(&SHOOTING_TEXTURES), 
            engine.load_refs(&RELOAD_TEXTURES),
        );

        let (shift, scale) = (0.27, 0.7);

        let soldier_sequences = EnemySequence::new(
            engine.load_refs(&SOLDIER_WALKING),
            engine.load_refs(&SOLDIER_SHOTING),
            engine.load_refs(&SOLDIER_PAIN),
            engine.load_refs(&SOLDIER_DYING),
        );

        // TODO: ejercico: añadir más enemigos clonando las secuencias
        let enemies = vec![
            Enemy::new(100, 5, (10.5, 3.5), soldier_sequences, shift, scale),
            // Enemy::new(100, 5, (12.5, 4.5), soldier_sequences.clone(), shift, scale),
        ];

        Box::new(Self {
            engine,
            player,
            enemies,
            weapon,
            is_over: false,
            background,
        })
    }

    pub fn run(&mut self) {
        'running: loop {
            if let Some(_exit) = self.handle_controls() {
                break 'running;
            }

            let events = self.update();

            self.handle_events(events);

            self.render();
        }
    }

    // TODO: ejercicio: escribir el cuerpo de esto
    fn render(&mut self) {

    }

    fn update(&mut self) -> Vec<Event> {
        let mut events = Vec::new();

        self.player.alive().not().then(|| events.push(Event::GameOver));

        let enemies = self.enemies.clone();

        self.enemies
            .iter_mut()
            .map(|soldier| soldier.update(&self.player, enemies.clone()))
            .flatten()
            .for_each(|event| events.push(event));

        self.weapon.update(&self.player, self.enemies.clone());
        self.player.update(&self.player.clone(), self.enemies.clone());

        events
    }

    fn handle_events(&mut self, events: Vec<Event>) {
        events.iter().for_each(|event| match event {
            Event::EnemyAttack(damage) => {
                self.engine.play_enemy_shoot();
                self.engine.play_player_pain();
                self.player.hit(*damage);
            }
            Event::GameOver => {
                self.is_over = true;
            }
        });
    }

    fn handle_controls(&mut self) -> Option<()> {
        match self.engine.poll() {
            Some(control) => match control {
                Controlls::Escape        => return Some(()), // shutdowns the game
                Controlls::ArrowUp       => self.player.walk(Mov::North, DELTA_TIME),
                Controlls::ArrowDown     => self.player.walk(Mov::South, DELTA_TIME),
                Controlls::ArrowLeft     => self.player.walk(Mov::West, DELTA_TIME),
                Controlls::ArrowRight    => self.player.walk(Mov::East, DELTA_TIME),
                Controlls::Spin(x, xrel) => self.spin_camera(x, xrel),
                Controlls::Enter         => self.shoot(),
            },
            None => {}
        }

        None
    }

    fn spin_camera(&mut self, x: i32, xrel: i32) {
        if x < MOUSE_BORDER_LEFT || x > MOUSE_BORDER_RIGHT {
            self.engine.center_mouse(HALF_WIDTH as i32, HALF_HEIGHT as i32);
        }

        let rel = max(-MOUSE_MAX_REL, min(MOUSE_MAX_REL, xrel));

        self.player.spin(rel as f32 * MOUSE_SENSITIVITY * DELTA_TIME);
    }

    fn shoot(&mut self) {
        self.weapon.shoot();
        self.engine.play_shoot();

        for enemy in self.enemies.iter_mut() {
            if enemy.alive() && enemy.hit(&self.player) {
                self.engine.play_pain_shout();

                enemy.receive_damage(self.weapon.damage());
            }
        }
    }
}