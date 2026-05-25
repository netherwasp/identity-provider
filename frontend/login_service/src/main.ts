import { bootstrapApplication } from '@angular/platform-browser';
import { appConfig } from './app/app.config';
import { App } from './app/app';
import init, { ensure_csrf } from './assets/wasm_package/api_wasm';

async function main() {
  await init('./assets/wasm_package/api_wasm_bg.wasm');
  await ensure_csrf();
  await bootstrapApplication(App, appConfig);
}

main().catch((err) => console.error(err));
