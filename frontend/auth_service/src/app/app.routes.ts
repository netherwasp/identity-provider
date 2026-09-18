import { Routes } from '@angular/router';
import { Login } from './component/page/login/login';
import { Register } from './component/page/register/register';
import { MatrixRain } from './component/animation/matrix-rain/matrix-rain';

export const routes: Routes = [
  { path: '', component: Login },
  { path: 'register', component: Register },
  { path: 'matrix-rain', component: MatrixRain },
  { path: '**', redirectTo: '', pathMatch: 'full' },
];
