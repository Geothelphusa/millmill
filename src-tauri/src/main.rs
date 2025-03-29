// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_store::{StoreBuilder, StoreCollection};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    name: String,
    start_date: String,
    end_date: String,
    color: String,
    is_dragging: bool,
    drag_offset: i64,
    drag_start_x: f64,
}

#[tauri::command]
async fn save_tasks(app_handle: tauri::AppHandle, tasks: Vec<Task>) -> Result<(), String> {
    let mut store = StoreBuilder::new(&app_handle, "tasks.json".parse().unwrap()).build();

    let mut map = HashMap::new();
    map.insert("tasks".to_string(), serde_json::to_value(tasks).map_err(|e| e.to_string())?);

    store.insert_map(map).map_err(|e| e.to_string())?;

    store.save().map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
async fn load_tasks(app_handle: tauri::AppHandle) -> Result<Vec<Task>, String> {
    let store = StoreBuilder::new(&app_handle, "tasks.json".parse().unwrap()).build();

    let mut store = app_handle.state::<StoreCollection>().get(&store.path).unwrap().lock().unwrap();
    store.load().map_err(|e| e.to_string())?;
    if let Some(tasks_value) = store.get("tasks") {
        serde_json::from_value(tasks_value.clone()).map_err(|e| e.to_string())
    } else {
        Ok(Vec::new())
    }

}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![save_tasks, load_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
