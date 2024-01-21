build:
	cargo build --all

server: build
	RUST_LOG=info cargo run --bin server

client: build
	RUST_LOG=info cargo run --bin kv