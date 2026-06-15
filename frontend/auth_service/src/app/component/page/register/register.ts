import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import {
  FormGroup,
  FormBuilder,
  Validators,
  ReactiveFormsModule,
  FormsModule,
} from '@angular/forms';
import { Router } from '@angular/router';
import { ButtonModule } from 'primeng/button';
import { InputGroupModule } from 'primeng/inputgroup';
import { InputGroupAddonModule } from 'primeng/inputgroupaddon';

@Component({
  selector: 'app-register',
  imports: [
    FormsModule,
    CommonModule,
    ReactiveFormsModule,
    ButtonModule,
    InputGroupModule,
    InputGroupAddonModule,
  ],
  standalone: true,
  templateUrl: './register.html',
  styleUrl: './register.scss',
})
export class Register {
  ngOnInit() {
    this.initColumns();
    this.startMatrixLoop();
  }

  // SIDE CONTENT FOR DESIGN ONLY ==========================================
  columns: { x: number; chars: string[]; speed: number; offset: number }[] = [];
  private readonly COL_WIDTH = 15;
  private readonly NUM_CHARS = 20;

  initColumns() {
    const containerWidth = 240; // match your side-content width
    const numCols = Math.floor(containerWidth / this.COL_WIDTH);

    this.columns = Array.from({ length: numCols }, (_, i) => ({
      x: i * this.COL_WIDTH,
      chars: Array.from({ length: this.NUM_CHARS }, () => this.getRandomChar()),
      speed: Math.random() * 3 + 2,
      offset: Math.random() * -100,
    }));
  }

  alphanum: string[] = [];

  getRandomX(): string {
    return `${Math.random() * 100}vw`;
  }

  getSpeed(): string {
    return `${Math.random() * 2 + 1}s`;
  }

  startMatrixLoop() {
    setInterval(() => {
      this.columns.forEach((col) => {
        const randomIndex = Math.floor(Math.random() * col.chars.length);
        col.chars[randomIndex] = this.getRandomChar();
      });
    }, 100);
  }

  getRandomChar(): string {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=';
    return chars.charAt(Math.floor(Math.random() * chars.length));
  }
  // =======================================================================
  registerForm: FormGroup;
  user: string | undefined;

  constructor(
    private router: Router,
    private fb: FormBuilder,
  ) {
    this.registerForm = this.fb.group({});
  }

  async onRegister() {
    if (this.registerForm.valid) {
      const { username, password } = this.registerForm.value;
      console.log('User Logging In', username);
    }
  }
}
