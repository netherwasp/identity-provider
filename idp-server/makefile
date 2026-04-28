.PHONY: wasm frontend server run debug

wasm:
	cd ../api_wasm && wasm-pack build --target web --out-dir ../login_page/src/assets/wasm_package

frontend:
	cd ../login_page && ng build --configuration production --output-path ../idp-server/src/priv

server:
	cd ../idp-server && cargo run

server-debug:
	cd ../idp-server && RUST_LOG=debug cargo run

run: wasm frontend server

debug: wasm frontend server-debug