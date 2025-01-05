import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';

export interface Todo {
  id: number;
  title: string;
  done: boolean;
}

@Injectable({
  providedIn: 'root',
})
export class TodoService {
  // Fetch todos from the backend
  async getTodos(): Promise<Todo[]> {
    return await invoke<Todo[]>('get_todos');
  }

  // Add a new todo
  async addTodo(title: string): Promise<Todo[]> {
    return await invoke<Todo[]>('add_todo', { title });
  }

  // Toggle the status of a todo
  async toggleTodo(id: number): Promise<Todo[]> {
    return await invoke<Todo[]>('toggle_todo', { id });
  }
}

