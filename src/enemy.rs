use std::ops::Mul;

use speedy2d::{dimen::Vector2, Graphics2D};

use crate::player::Player;

#[derive(Clone)]
pub struct Enemy {
    inner: Player,
    speed: f32,
    radius: f32,
}

impl Enemy {
    pub fn new(pos: Vector2<f32>) -> Self {
        let mut inner = Player::new();
        inner.pos = pos;
        Self {
            inner,
            speed: 0.3,
            radius: 20.,
        }
    }

    pub fn move_to(&mut self, pos: &Vector2<f32>) { 
        const EPS: f32 = 500.;
        let diff = pos - self.inner.pos;
        if diff.magnitude_squared() < EPS { return; }
        let angle = diff.y.atan2(diff.x);
        let dir_vec = Vector2::new(angle.cos(), angle.sin());
        self.inner.dir = angle;
        self.inner.move_(dir_vec.mul(self.speed));
    }

    pub fn update(&mut self, dt: f32) {
        self.inner.update(dt, None);
    }

    pub fn render(&mut self, graphics: &mut Graphics2D) {
        self.inner.render(graphics);
    }

    pub fn check_collision(&mut self, others: &Vec<Enemy>) {
        others.iter().for_each(|other| {
            let dist = self.inner.pos - other.inner.pos;
            let dist_magnitude = dist.magnitude();
            if dist_magnitude < 2. * self.radius {
                let norm = dist.normalize().unwrap_or(Vector2::new(1., 1.));
                let offset = norm.mul(2. * self.radius - dist_magnitude);
                self.inner.pos += offset;
            }
        });
    }
}
