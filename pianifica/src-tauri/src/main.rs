#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

struct AppState {
    todos: Mutex<Vec<Todo>>,
}

#[tauri::command]
fn get_todos(state: State<AppState>) -> Vec<Todo> {
    state.todos.lock().unwrap().clone()
}

#[tauri::command]
fn add_todo(state: State<AppState>, title: String) -> Vec<Todo> {
    let mut todos = state.todos.lock().unwrap();
    let new_id = todos.len() as u32 + 1;
    todos.push(Todo {
        id: new_id,
        title,
        done: false,
    });
    todos.clone()
}

#[tauri::command]
fn toggle_todo(state: State<AppState>, id: u32) -> Vec<Todo> {
    let mut todos = state.todos.lock().unwrap();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.done = !todo.done;
    }
    todos.clone()
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            todos: Mutex::new(vec![]),
        })
        .invoke_handler(tauri::generate_handler![get_todos, add_todo, toggle_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
