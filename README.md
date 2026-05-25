# identity-provider
An Identity Server Developed in Rust

# For Initialization
### Create .env
1.  Create A .env inside idp-server
```env
    SUPER_ADMIN_URL = postgres://postgres:sample_secret@localhost:5432/postgres
    IDP_ADMIN_URL = postgres://idp_admin:sample_secret@localhost:5432/sample_db
```


# For Debugging
```cmd
RUST_LOG=debug cargo run
```

# For Login page
### Building wasm package
1. Open the Folder directory api_wasm
2. In the src/lib.rs check the corresponding HOST_IDP match it with the domain of the idp-server package
3. On the terminal open the api_wasm directory and run this command 
#### Note this build command compiles the api_wasm to be used in login_service
```
wasm-pack build --target web --out-dir ../frontend/login_service/src/assets/wasm_package
```


### Building Login Page (Angular)
1. Check the angular.json in the login_service directory make sure that the angular.json on `build` have this kind of `assets`
```json
"assets": [
    "src/assets",
    {
    "glob": "**/*",
    "input": "login-service/src/assets/wasm_package",
    "output": "assets"
    } 
],       
```
2. Open the terminal and go to the login_service directory then run this command:
#### Note this build command compiles the login_service to be served directly to the idp-server binary
```
ng build --configuration production --base-href /auth/ --deploy-url /auth/ --output-path ../../idp-server/src/priv/login_service
```
   

