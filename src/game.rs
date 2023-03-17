use speedy2d::window::VirtualKeyCode;
use speedy2d::{Graphics2D, dimen::Vector2};

use rand::Rng;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::keysystem::KeySystem;

pub struct Game {
    pub player: Player,
    pub keysystem: KeySystem,
    pub mouse_position: Vector2<f32>,
    pub enemies: Vec<Enemy>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            keysystem: KeySystem::new(),
            mouse_position: Vector2::ZERO,
            enemies: vec![],
        }
    }

    pub fn check_keypress(&mut self) {
        let mut dx = 0.;
        let mut dy = 0.;
        for key in self.keysystem.pressed_key.clone().iter() {
            match key {
                VirtualKeyCode::Z => { dy = -1. },
                VirtualKeyCode::S => { dy = 1.},
                VirtualKeyCode::Q => { dx = -1. },
                VirtualKeyCode::D => { dx = 1. },
                VirtualKeyCode::E => self.add_random_enemy(),
                _ => {},
            };
        }
        self.player.move_(Vector2::new(dx,dy))
    }

    pub fn set_mouse_position(&mut self, position: Vector2<f32>) {
        self.mouse_position = position;
    }

    pub fn update(&mut self, dt: f32) {
        self.check_keypress();
        self.player.update(dt, Some(self.mouse_position));
        let enemies_n = self.enemies.len();
        let mut i = 0;
        while i < enemies_n {
            let mut other_enemies = self.enemies.clone();
            other_enemies.remove(i);
            let enemy = self.enemies.get_mut(i).unwrap();
            enemy.move_to(&self.player.pos);
            enemy.update(dt);
            enemy.check_collision(&other_enemies);
            i += 1;
        }
    }
    
    pub fn render(&mut self, graphics: &mut Graphics2D) {
        self.player.render(graphics);
        self.enemies.iter_mut().for_each(|e| e.render(graphics));
        self.player.display_stats(graphics);

    }

    pub fn add_random_enemy(&mut self) {
        let mut rng = rand::thread_rng();
        let enemy = Enemy::new(Vector2::new(200. + rng.gen::<f32>() * 200., 200.));
        self.enemies.push(enemy);
    }

}
