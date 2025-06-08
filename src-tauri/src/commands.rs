use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::physics::{BallPosition, PhysicsWorld};

#[derive(Serialize, Deserialize)]
pub struct AppState {
    pub physics_world: Mutex<PhysicsWorld>,
}

#[tauri::command]
pub fn reset_simulation(state: tauri::State<AppState>) {
    state.physics_world.lock().unwrap().reset();
}

#[tauri::command]
pub fn get_ball_position(state: tauri::State<AppState>) -> BallPosition {
    let world = state.physics_world.lock().unwrap();
    let [x, y, z] = world.ball.position;
    BallPosition { x, y, z }
}

#[tauri::command]
pub fn physics_step(state: tauri::State<AppState>, dt: f32) {
    state.physics_world.lock().unwrap().step(dt);
}
