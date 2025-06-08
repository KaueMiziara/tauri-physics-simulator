pub mod commands;
pub mod physics;

use physics::PhysicsWorld;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(commands::AppState {
            physics_world: std::sync::Mutex::new(PhysicsWorld::new()),
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_ball_position,
            commands::physics_step,
            commands::reset_simulation
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
