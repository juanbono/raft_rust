FROM rust:1.75 as builder
WORKDIR /usr/src/raft_kv
COPY . .
COPY config.yaml /usr/src/raft_kv/config.yaml
RUN cargo install --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY --from=builder /usr/src/raft_kv/config.yaml /usr/local/bin/config.yaml
ENV RUST_LOG=info
ENV PEER_ID=1
ENV SERVER_CONFIG=/usr/local/bin/config.yaml
CMD ["server"]