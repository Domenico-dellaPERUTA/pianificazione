import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';

export interface Todo {
  id: number;
  title: string;
  done: boolean;
  priority: 'Low' | 'Medium' | 'High';
}

@Injectable({
  providedIn: 'root',
})
export class TodoService {
  async getTodos(): Promise<Todo[]> {
    return await invoke<Todo[]>('get_todos');
  }

  async addTodo(title: string, priority: 'Low' | 'Medium' | 'High'): Promise<Todo[]> {
    return await invoke<Todo[]>('add_todo', { title, priority });
  }

  async toggleTodo(id: number): Promise<Todo[]> {
    return await invoke<Todo[]>('toggle_todo', { id });
  }

  async updatePriority(id: number, priority: 'Low' | 'Medium' | 'High'): Promise<Todo[]> {
    return await invoke<Todo[]>('update_priority', { id, priority });
  }

  async resetTodos(): Promise<Todo[]> {
    await invoke('reset_todos');
    return this.getTodos();
  }
}
