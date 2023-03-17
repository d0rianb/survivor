use std::{ops::Mul, borrow::Borrow};

use speedy2d::{dimen::Vector2, Graphics2D, color::Color, font::{Font, TextLayout, TextOptions}};

#[derive(Clone)]
pub struct Player {
    pub pos: Vector2<f32>,
    pub dir: f32,
    pub speed: Vector2<f32>,
    pub acc: Vector2<f32>,
    font: Font
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vector2::ZERO,
            speed: Vector2::ZERO,
            acc: Vector2::ZERO,
            dir: 0.,
            font: Font::new(include_bytes!("../resources/font/CourierRegular.ttf")).unwrap(),
        }
    }

    pub fn move_(&mut self, acc: Vector2<f32>) {
        const ACC_VALUE: f32 = 1. / 10.;
        self.acc = acc.mul(ACC_VALUE)
    }

    pub fn decelerate(&mut self) {
        const FRICTION_COEFF: f32 = 0.1;
        const EPS: f32 = 1e-3;
        let friction = self.speed.mul(FRICTION_COEFF);
        self.acc = self.acc - friction;
        if self.speed.magnitude() < EPS { self.speed = Vector2::ZERO; }
    }

    pub fn set_dir(&mut self, pos: Vector2<f32>) {
        let dist = pos - self.pos;
        self.dir = dist.y.atan2(dist.x)
    }

    pub fn update(&mut self, dt: f32, mouse_pos: Option<Vector2<f32>>) {
        const MAX_SPEED: f32 = 0.3;
        if let Some(pos) = mouse_pos { 
            self.set_dir(pos);
        }

        self.decelerate();

        self.speed = Vector2 {
            x: (self.acc.x * dt + self.speed.x).clamp(-MAX_SPEED, MAX_SPEED),
            y: (self.acc.y * dt + self.speed.y).clamp(-MAX_SPEED, MAX_SPEED),
        };

        self.pos = Vector2 {
            x: self.acc.x * dt * dt / 2. + self.speed.x * dt + self.pos.x,
            y: self.acc.y * dt * dt / 2. + self.speed.y * dt + self.pos.y,
        }
    }

    pub fn display_stats(&self, graphics: &mut Graphics2D) {
        let text = format!("Pos: {:?}
            Speed: {:?} 
            Acc: {:?}
            Dir: {:?}", 
            self.pos, self.speed, self.acc, self.dir
        );
        let ftb = self.font.borrow().layout_text(&text, 15., TextOptions::default());
        graphics.draw_text(Vector2::new(5., 5.), Color::WHITE, &ftb)
    }

    pub fn render(&self, graphics: &mut Graphics2D) {
        graphics.draw_circle(self.pos, 20., Color::WHITE);
        graphics.draw_line(self.pos, self.pos + Vector2::new(self.dir.cos(), self.dir.sin()).mul(50.), 2., Color::WHITE);
    }

}
