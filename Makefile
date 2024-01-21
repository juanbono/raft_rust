build:
	cargo build --all

server: build
	RUST_LOG=info cargo run --bin server

client: build
	RUST_LOG=info cargo run --bin kv

fmt:
	cargo fmt --all

lint:
	cargo clippy --all

test:
	cargo test --all
