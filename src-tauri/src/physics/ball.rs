use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Ball {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub radius: f32,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
            radius: 0.5,
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
}
