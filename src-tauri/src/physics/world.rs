use serde::{Deserialize, Serialize};

use super::ball::Ball;

#[derive(Default, Serialize, Deserialize)]
pub struct PhysicsWorld {
    pub ball: Ball,
    pub gravity: [f32; 3],
    pub floor_y: f32,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            ball: Ball::new(),
            gravity: [0.0, -9.8, 0.0],
            floor_y: 0.0,
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.ball.apply_gravity(self.gravity, dt);
        self.ball.update_position(dt);
        self.ball.check_floor_collision(self.floor_y);
    }

    pub fn reset(&mut self) {
        self.ball = Ball::new();
    }
}
