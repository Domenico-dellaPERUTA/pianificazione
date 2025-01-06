use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::sync::Mutex;
use tauri::State;
use dirs::data_local_dir;

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

struct AppState {
    todos: Mutex<Vec<Todo>>,
    file_path: String,
}

impl AppState {
    fn load_todos(&self) -> Vec<Todo> {
        let file_path = &self.file_path;
        if let Ok(mut file) = File::open(file_path) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(todos) = serde_json::from_str(&contents) {
                    return todos;
                }
            }
        }
        Vec::new()
    }

    fn save_todos(&self, todos: &Vec<Todo>) {
        let file_path = &self.file_path;
        if let Ok(mut file) = File::create(file_path) {
            if let Ok(contents) = serde_json::to_string_pretty(todos) {
                let _ = file.write_all(contents.as_bytes());
            }
        }
    }
}

#[tauri::command]
fn get_todos(state: State<AppState>) -> Vec<Todo> {
    let todos = state.load_todos();
    todos
}

#[tauri::command]
fn add_todo(state: State<AppState>, title: String) -> Vec<Todo> {
    let mut todos = state.todos.lock().unwrap();
    let new_id = if todos.is_empty() {
        1
    } else {
        todos.iter().map(|t| t.id).max().unwrap() + 1
    };

    let new_todo = Todo {
        id: new_id,
        title,
        done: false,
    };
    todos.push(new_todo);
    state.save_todos(&todos);
    todos.clone()
}

#[tauri::command]
fn toggle_todo(state: State<AppState>, id: u32) -> Vec<Todo> {
    let mut todos = state.todos.lock().unwrap();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.done = !todo.done;
    }
    state.save_todos(&todos);
    todos.clone()
}

fn main() {
    let file_path = std::env::current_dir()
        //data_local_dir()
        .expect("Failed to get local data directory")
        //.join("pianifica")
        .join("todos.json");

    fs::create_dir_all(file_path.parent().unwrap()).expect("Failed to create app data directory");

    tauri::Builder::default()
        .manage(AppState {
            todos: Mutex::new(Vec::new()),
            file_path: file_path.to_string_lossy().to_string(),
        })
        .invoke_handler(tauri::generate_handler![get_todos, add_todo, toggle_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
