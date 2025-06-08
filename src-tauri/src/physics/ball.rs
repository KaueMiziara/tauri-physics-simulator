use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Ball {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub radius: f32,
    pub restitution: f32,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            position: [0.0, 5.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
            radius: 0.5,
            restitution: 0.8,
        }
    }

    pub fn apply_gravity(&mut self, gravity: [f32; 3], dt: f32) {
        self.velocity[0] += gravity[0] * dt;
        self.velocity[1] += gravity[1] * dt;
        self.velocity[2] += gravity[2] * dt;
    }

    pub fn update_position(&mut self, dt: f32) {
        self.position[0] += self.velocity[0] * dt;
        self.position[1] += self.velocity[1] * dt;
        self.position[2] += self.velocity[2] * dt;
    }

    pub fn check_floor_collision(&mut self, floor_y: f32) -> bool {
        if self.position[1] - self.radius <= floor_y {
            self.position[1] = floor_y + self.radius;
            self.velocity[1] = -self.velocity[1] * self.restitution;

            self.velocity[0] *= 0.99;
            self.velocity[2] *= 0.99;
            true
        } else {
            false
        }
    }
}
