# identity-provider
An Identity Server Developed in Rust

# For Login page
### Building wasm package
1. Open the Folder directory api_wasm
2. In the src/lib.rs check the corresponding HOST_IDP match it with the domain of the idp-server package
3. On the terminal open the api_wasm directory and run this command 
#### Note this build command compiles the api_wasm to be used in login_page
```
wasm-pack build --target web --out-dir ../login_page/src/assets/wasm_package
```


### Building Login Page (Angular)
1. Check the angular.json in the login_page directory make sure that the angular.json on `build` have this kind of `assets`
```json
"assets": [
    "src/assets",
    {
    "glob": "**/*",
    "input": "login-page/src/assets/wasm_package",
    "output": "assets"
    } 
],       
```
2. Open the terminal and go to the login_page directory then run this command:
#### Note this build command compiles the login_page to be served directly to the idp-server binary
```
ng build --configuration production  --output-path ../idp-server/src/priv
```
   

