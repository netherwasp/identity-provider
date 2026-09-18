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
import { MatrixRain } from '../../animation/matrix-rain/matrix-rain';

@Component({
  selector: 'app-register',
  imports: [CommonModule, ReactiveFormsModule, ButtonModule, MatrixRain],
  standalone: true,
  templateUrl: './register.html',
  styleUrl: './register.scss',
})
export class Register implements OnInit {
  ngOnInit() {}
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
        preferred_username: ['', [Validators.required, Validators.minLength(6)]],
        email: ['', [Validators.required, Validators.email]],
        given_name: ['', [Validators.required, Validators.pattern('^[a-zA-Z]+$')]],
        family_name: ['', [Validators.required, Validators.pattern('^[a-zA-Z]+$')]],
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

    console.log('Registration payload', JSON.stringify(payload));
    // TODO: wire up HttpClient call to the Axum registration endpoint once
    // the route is confirmed.
  }
}
