use serde::{Deserialize, Serialize};

use super::ball::Ball;

#[derive(Default, Serialize, Deserialize)]
pub struct PhysicsWorld {
    pub ball: Ball,
    pub gravity: [f32; 3],
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            ball: Ball::new(),
            gravity: [0.0, -9.8, 0.0],
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.ball.apply_gravity(self.gravity, dt);
        self.ball.update_position(dt);
    }
}
