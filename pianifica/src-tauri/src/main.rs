use std::sync::{Arc, Mutex};
use tauri::{State};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct Todo {
    id: u32,
    title: String,
    priority: String,
    done: bool,
}

struct AppState {
    todos: Arc<Mutex<Vec<Todo>>>,
}

#[tauri::command]
fn get_todos(state: State<AppState>) -> Vec<Todo> {
    // Blocca il mutex e clona i dati
    let todos = state.todos.lock().unwrap();
    todos.clone()
}

#[tauri::command]
fn add_todo(state: State<AppState>, title: String, priority: String) -> Vec<Todo> {
    let mut todos = state.todos.lock().unwrap();

    // Crea un nuovo TODO con ID unico
    let id = if let Some(last) = todos.last() {
        last.id + 1
    } else {
        1
    };
    todos.push(Todo {
        id,
        title,
        priority,
        done: false,
    });

    // Rilascia il guard prima di restituire i todo
    drop(todos);
    get_todos(state)
}

#[tauri::command]
fn toggle_todo(state: State<AppState>, id: u32) -> Vec<Todo> {
    let mut todos = state.todos.lock().unwrap();

    // Alterna lo stato di completamento
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.done = !todo.done;
    }

    // Rilascia il guard prima di restituire i todo
    drop(todos);
    get_todos(state)
}

#[tauri::command]
fn reset_todos(state: State<AppState>) -> Vec<Todo> {
    let mut todos = state.todos.lock().unwrap();

    // Svuota la lista
    todos.clear();

    // Rilascia il guard prima di restituire i todo
    drop(todos);
    get_todos(state)
}

fn main() {
    let app_state = AppState {
        todos: Arc::new(Mutex::new(Vec::new())),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_todos,
            add_todo,
            toggle_todo,
            reset_todos
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
