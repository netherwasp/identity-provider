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
import { User, At, Lock, Unlock, Phone } from '@primeicons/angular';
import { MatrixRain } from '../../animation/matrix-rain/matrix-rain';
import { COUNTRIES, Country } from '../../../interfaces/register-interface';

@Component({
  selector: 'app-register',
  imports: [
    CommonModule,
    FormsModule,
    ReactiveFormsModule,
    ButtonModule,
    MatrixRain,
    IftaLabelModule,
    InputTextModule,
    InputGroupModule,
    InputGroupAddonModule,
    SelectModule,
    User,
    At,
    Lock,
    Unlock,
    Phone,
  ],
  standalone: true,
  templateUrl: './register.html',
  styleUrl: './register.scss',
})
export class Register {
  registerForm: FormGroup;
  countries: Country[] = COUNTRIES;

  constructor(
    private router: Router,
    private fb: FormBuilder,
  ) {
    this.registerForm = this.fb.group(
      {
        username: ['', [Validators.required, Validators.minLength(6)]],
        email: ['', [Validators.required, Validators.email]],
        given_name: ['', [Validators.required, Validators.pattern('^[a-zA-Z]+$')]],
        family_name: ['', [Validators.required, Validators.pattern('^[a-zA-Z]+$')]],
        country_code: [null as Country | null, Validators.required],
        phone_number: ['', [Validators.required, Validators.pattern('^[0-9 ]{6,15}$')]],
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

  getCountryFlag(iso: string): string {
    return iso
      .toUpperCase()
      .split('')
      .map((char) => String.fromCodePoint(127397 + char.charCodeAt(0)))
      .join('');
  }

  async onRegister() {
    if (this.registerForm.invalid) {
      this.registerForm.markAllAsTouched();
      return;
    }
    const { confirmPassword, password, country_code, phone_number, ...rest } =
      this.registerForm.value;

    const national = String(phone_number).replace(/\s+/g, '').replace(/^0+/, '');
    const claims = {
      ...rest,
      phone_number: `${country_code.dial_code}${national}`, // standard OIDC claim name
    };

    // const contact_number = `${country_code.dial_code}${phone_number}`;
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
