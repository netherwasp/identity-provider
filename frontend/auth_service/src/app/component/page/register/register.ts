import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import {
  AbstractControlOptions,
  AbstractControl,
  ValidationErrors,
  ValidatorFn,
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
import { IftaLabelModule } from 'primeng/iftalabel';
import { InputTextModule } from 'primeng/inputtext';
import { SelectModule } from 'primeng/select';
import { MessageModule } from 'primeng/message';
import { MatrixRain } from '../../animation/matrix-rain/matrix-rain';
import { COUNTRIES, Country } from '../../../interfaces/register-interface';

export const confirmPasswordValidator: ValidatorFn = (
  group: AbstractControl,
): ValidationErrors | null => {
  return group.get('password')?.value === group.get('confirmPassword')?.value
    ? null
    : { passwordMismatch: true };
};

export const fullNameValidator: ValidatorFn = (group: AbstractControl): ValidationErrors | null => {
  return !group.get('given_name')?.invalid &&
    group.get('given_name')?.value?.trim()?.length > 0 &&
    !group.get('family_name')?.invalid &&
    group.get('family_name')?.value?.trim()?.length > 0
    ? null
    : { fullNameInvalid: true };
};

@Component({
  selector: 'app-register',
  imports: [
    CommonModule,
    FormsModule,
    ReactiveFormsModule,
    ButtonModule,
    MatrixRain,
    MessageModule,
    IftaLabelModule,
    InputTextModule,
    InputGroupModule,
    InputGroupAddonModule,
    SelectModule,
  ],
  standalone: true,
  templateUrl: './register.html',
  styleUrl: './register.scss',
})
export class Register {
  registerForm: FormGroup;
  countries: Country[] = COUNTRIES;

  async onLogIn() {
    this.router.navigate(['/']);
  }

  constructor(
    private router: Router,
    private fb: FormBuilder,
  ) {
    this.registerForm = this.fb.group(
      {
        username: ['', [Validators.required, Validators.minLength(6)]],
        email: ['', [Validators.required, Validators.email]],
        given_name: ['', [Validators.required, Validators.pattern(/^[a-zA-Z\s]+$/)]],
        family_name: ['', [Validators.required, Validators.pattern(/^[a-zA-Z\s]+$/)]],
        country_code: [null as Country | null, Validators.required],
        phone_number: ['', [Validators.required, Validators.pattern(/^[0-9]{6,15}$/)]],
        password: ['', [Validators.required, Validators.minLength(6)]],
        confirmPassword: ['', Validators.required],
      },
      { validators: [confirmPasswordValidator, fullNameValidator] },
    );
  }

  onPhoneInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const digitsOnly = input.value.replace(/[^0-9]/g, ''); // strip anything that's not 0-9

    if (input.value !== digitsOnly) {
      input.value = digitsOnly;
      this.registerForm.get('phone_number')?.setValue(digitsOnly, { emitEvent: false });
    }
  }

  async onRegister() {
    if (this.registerForm.invalid) {
      this.registerForm.markAllAsTouched();
      return;
    }
    const { confirmPassword, password, country_code, phone_number, ...rest } =
      this.registerForm.value;

    const national = String(phone_number).replace(/\s+/g, '').replace(/^0+/, '');
    const payload = {
      ...rest,
      password,
      phone_number: `${country_code.dial_code}${national}`, // standard OIDC claim name
    };

    console.log('Registration payload', JSON.stringify(payload));
  }
}
