import { Routes } from '@angular/router';
import { LoginPage } from './login/login-page';

export const routes: Routes = [
    { path: 'login' , component: LoginPage},
    {path: 'register', component: Register}
    { path: '**', redirectTo: '', pathMatch: 'full'}
];
