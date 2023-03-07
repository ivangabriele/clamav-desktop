test:
	cd ./src-tauri && cargo test --workspace -- --nocapture

test-cli:
	cd ./src-tauri/cli &&	cargo test -- --nocapture

test-filer:
	cd ./src-tauri/filer &&	cargo test -- --nocapture

test-jest:
	cd ./src-tauri/jest &&	cargo test -- --nocapture

test-watch:
	cd ./src-tauri && cargo watch -x "test --workspace -- --nocapture"
