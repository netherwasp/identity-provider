import { Routes } from '@angular/router';
import { Login } from './component/page/login/login';
import { Register } from './component/page/register/register';

export const routes: Routes = [
  { path: '', component: Login },
  { path: 'register', component: Register },
  { path: '**', redirectTo: '', pathMatch: 'full' },
];
