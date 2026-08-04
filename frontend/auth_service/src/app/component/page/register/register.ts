import { CommonModule } from '@angular/common';
import { Component, OnInit } from '@angular/core';
import {
  AbstractControlOptions,
  AbstractControl,
  ValidationErrors,
  ValidatorFn,
  FormGroup,
  FormBuilder,
  Validators,
  ReactiveFormsModule,
} from '@angular/forms';
import { Router } from '@angular/router';
import { ButtonModule } from 'primeng/button';

@Component({
  selector: 'app-register',
  imports: [CommonModule, ReactiveFormsModule, ButtonModule],
  standalone: true,
  templateUrl: './register.html',
  styleUrl: './register.scss',
})
export class Register implements OnInit {
  ngOnInit() {
    this.initColumns();
    this.startMatrixLoop();
  }

  // SIDE CONTENT FOR DESIGN ONLY ==========================================
  columns: { x: number; chars: string[]; speed: number; offset: number }[] = [];
  private readonly COL_WIDTH = 15;
  private readonly NUM_CHARS = 20;

  initColumns() {
    const containerWidth = 240;
    const numCols = Math.floor(containerWidth / this.COL_WIDTH);
    this.columns = Array.from({ length: numCols }, (_, i) => ({
      x: i * this.COL_WIDTH,
      chars: Array.from({ length: this.NUM_CHARS }, () => this.getRandomChar()),
      speed: Math.random() * 3 + 2,
      offset: Math.random() * -100,
    }));
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

  // Form controls are named after standard OIDC claims so the payload can be
  // sent straight through without a remapping step. `password` /
  // `confirmPassword` are the only non-claim fields (credentials, not
  // identity data) and are stripped before building the claims payload.
  registerForm: FormGroup;

  constructor(
    private router: Router,
    private fb: FormBuilder,
  ) {
    this.registerForm = this.fb.group(
      {
        preferred_username: ['', [Validators.required, Validators.minLength(3)]],
        email: ['', [Validators.required, Validators.email]],
        given_name: ['', Validators.required],
        family_name: ['', Validators.required],
        password: ['', [Validators.required, Validators.minLength(6)]],
        confirmPassword: ['', Validators.required],
      },
      { validators: this.passwordsMatch } as AbstractControlOptions,
    );
  }

  private passwordsMatch: ValidatorFn = (group: AbstractControl): ValidationErrors | null => {
    const password = group.get('password')?.value;
    const confirmPassword = group.get('confirmPassword')?.value;
    return password === confirmPassword ? null : { passwordMismatch: true };
  };

  async onRegister() {
    if (this.registerForm.invalid) {
      this.registerForm.markAllAsTouched();
      return;
    }

    const { confirmPassword, password, ...claims } = this.registerForm.value;

    // Standard OIDC claims payload — sent to the IDP's registration
    // endpoint. `password` travels alongside it for credential creation but
    // is not itself a claim.
    const payload = {
      claims,
      password,
    };

    console.log('Registration payload', payload);
    // TODO: wire up HttpClient call to the Axum registration endpoint once
    // the route is confirmed.
  }
}
