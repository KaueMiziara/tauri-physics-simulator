pub mod commands;
pub mod physics;

use commands::{get_ball_position, physics_step, AppState};
use physics::PhysicsWorld;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            physics_world: std::sync::Mutex::new(PhysicsWorld::new()),
        })
        .invoke_handler(tauri::generate_handler![get_ball_position, physics_step])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
