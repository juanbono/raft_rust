build:
	cargo build --all

# Run server with default config
# make server peer=1
# make server peer=2
server: build
	RUST_LOG=info PEER_ID=$(peer) SERVER_CONFIG=config.yaml cargo run --bin server

client: build
	RUST_LOG=info cargo run --bin kv

fmt:
	cargo fmt --all

lint:
	cargo clippy --all

test:
	cargo test --all

docker-build:
	docker build . -t server -f docker/Dockerfile

docker-run:
	docker run -p 8080:8080 -it --rm --name kv_server server
