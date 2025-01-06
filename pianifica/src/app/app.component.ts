import { CommonModule } from '@angular/common';
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms'; // Modulo Forms
import { RouterOutlet } from '@angular/router';
import { Todo, TodoService } from './services/todo.service';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet, FormsModule],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})

export class AppComponent implements OnInit {
  todos: Todo[] = [];
  newTodo = '';
  newPriority: 'Low' | 'Medium' | 'High' = 'Low'; // Assicurati che sia un valore valido

  constructor(private todoService: TodoService) {}

  async ngOnInit() {
    await this.loadTodos();
  }

  async loadTodos() {
    this.todos = await this.todoService.getTodos();
    this.sortTodosByPriority();
  }

  sortTodosByPriority() {
    this.todos.sort((a, b) => this.priorityOrder(a.priority, b.priority));
  }

  priorityOrder(a: 'Low' | 'Medium' | 'High', b: 'Low' | 'Medium' | 'High'): number {
    const order = { High: 1, Medium: 2, Low: 3 };
    return order[a] - order[b];
  }

  async addTodo() {
    if (this.newTodo.trim()) {
      this.todos = await this.todoService.addTodo(this.newTodo.trim(), this.newPriority);
      this.newTodo = '';
      this.sortTodosByPriority(); // Riorganizza la lista dopo l'aggiunta
    }
  }

  async toggleTodo(id: number) {
    this.todos = await this.todoService.toggleTodo(id);
  }

  async resetTodos() {
    this.todos = await this.todoService.resetTodos();
    this.sortTodosByPriority(); // Riorganizza la lista dopo il reset
  }
}

