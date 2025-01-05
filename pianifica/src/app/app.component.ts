import { CommonModule } from '@angular/common';
import { Component, OnInit } from '@angular/core';
import { Todo, TodoService } from './services/todo.service';
import { RouterOutlet } from '@angular/router';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet,FormsModule],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent implements OnInit {
  todos: Todo[] = [];
  newTodo = '';

  constructor(private todoService: TodoService) {}

  async ngOnInit() {
    await this.loadTodos();
  }

  async loadTodos() {
    this.todos = await this.todoService.getTodos();
  }

  async addTodo() {
    if (this.newTodo.trim()) {
      this.todos = await this.todoService.addTodo(this.newTodo.trim());
      this.newTodo = '';
    }
  }

  async toggleTodo(id: number) {
    this.todos = await this.todoService.toggleTodo(id);
  }
}


