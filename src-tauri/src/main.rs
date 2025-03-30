// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::Mutex;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
  name: String,
  description: String,
  due_date: String,
  status: String,
}

#[derive(Debug, Default)]
struct AppState {
  tasks: Mutex<Vec<Task>>,
}

#[tauri::command]
fn add_task(
  state: State<'_, AppState>,
  name: String,
  description: String,
  due_date: String,
  status: String,
) -> Result<(), String> {
  let mut tasks = state.tasks.lock().unwrap();
  let new_task = Task {
    name,
    description,
    due_date,
    status,
  };
  tasks.push(new_task);
  println!("Tasks: {:?}", tasks);
  Ok(())
}

#[tauri::command]
fn save_tasks(state: State<'_, AppState>) -> Result<(), String> {
  let tasks = state.tasks.lock().unwrap();
  let json = serde_json::to_string(&*tasks).map_err(|e| e.to_string())?;
  let mut file = File::create("tasks.json").map_err(|e| e.to_string())?;
  file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
  Ok(())
}

#[tauri::command]
fn load_tasks(state: State<'_, AppState>) -> Result<Vec<Task>, String> {
  let mut file = File::open("tasks.json").map_err(|e| e.to_string())?;
  let mut json = String::new();
  file.read_to_string(&mut json).map_err(|e| e.to_string())?;
  let tasks: Vec<Task> = serde_json::from_str(&json).map_err(|e| e.to_string())?;
  let mut app_state_tasks = state.tasks.lock().unwrap();
  *app_state_tasks = tasks.clone();
  Ok(tasks)
}


fn main() {
  tauri::Builder::default()
    .manage(AppState::default())
    .invoke_handler(tauri::generate_handler![add_task, save_tasks, load_tasks])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
