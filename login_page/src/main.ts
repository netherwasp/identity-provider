import { bootstrapApplication } from '@angular/platform-browser';
import { appConfig } from './app/app.config';
import { App } from './app/app';
import init from './assets/wasm_package/api_wasm';

async function main() {
  await init('./assets/wasm_package/api_wasm_bg.wasm');
  await bootstrapApplication(App, appConfig);
}

main().catch((err) => console.error(err));
