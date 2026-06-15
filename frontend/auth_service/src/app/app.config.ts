import { ApplicationConfig, provideBrowserGlobalErrorListeners } from '@angular/core';
import { provideRouter } from '@angular/router';
import { routes } from './app.routes';
import { providePrimeNG } from 'primeng/config';
import Identifier from '../assets/themes/identifier';

document.documentElement.classList.add('app-dark'); 

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter(routes),
    providePrimeNG({
      theme: {
        preset: Identifier,
        options: {
          prefix: 'i',
          darkModeSelector: '.app-dark',
          cssLayer: false,
        },
      },
    }),
  ],
};