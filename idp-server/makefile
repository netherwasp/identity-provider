.PHONY: wasm frontend server run debug

wasm:
	cd ../api_wasm && wasm-pack build --target web --out-dir ../login_page/src/assets/wasm_package

frontend:
	cd ../login_page && ng build --configuration production --base-href /auth/ --deploy-url /auth/ --output-path ../idp-server/src/priv

server:
	cd ../idp-server && cargo run

server-debug:
	cd ../idp-server && RUST_LOG=debug cargo run

run: wasm frontend server

debug: wasm frontend server-debug

# DATABASE CREATION
DB_NAME=idp_db
USER=idp_admin
PASSWORD=poiuytre@73!

.PHONY: createdb dropdb dropuser createuser psql

createdb: 
	psql -U postgres -c "CREATE DATABASE $(DB_NAME) OWNER $(USER);"

dropdb:
	psql -U postgres -c "DROP DATABASE $(DB_NAME);"

dropuser:
	psql -U postgres -c "DROP USER $(USER);"

createuser:
	psql -U postgres -c "CREATE USER $(USER) WITH PASSWORD '$(PASSWORD)';"
	psql -U postgres -c "ALTER ROLE $(USER) WITH SUPERUSER;"

psql:
	psql -d $(DB_NAME) -U $(USER)
