# rust-devin

A Cargo workspace. All external dependency versions are declared in the root
[`Cargo.toml`](./Cargo.toml) under `[workspace.dependencies]`, and member
crates reference them with `dep = { workspace = true }`.

## Crates

| Crate | Kind | Purpose |
| --- | --- | --- |
| [`crates/common`](./crates/common) | library | Shared types, error enum, config, tracing setup. |
| [`crates/proto`](./crates/proto) | library | Protobuf definitions + generated tonic-grpc bindings. |
| [`crates/web`](./crates/web) | binary | Axum HTTP server + tonic-grpc Greeter server. |

## Prerequisites

- Rust (stable, edition 2021+)
- `protoc` (protobuf compiler) — required at build time by `tonic-build`.
  - Ubuntu/Debian: `sudo apt-get install -y protobuf-compiler`
  - macOS: `brew install protobuf`

## Build & test

```bash
cargo build
cargo test
```

## Run the web binary

The `web` binary exposes HTTP on `AppConfig::port` (default `3000`) and gRPC
on `port + 1` (default `3001`):

```bash
cargo run -p web
# HTTP
curl http://localhost:3000/health
# gRPC (using grpcurl, requires server reflection to be turned on for this to
# work without proto files; otherwise use a generated client)
grpcurl -plaintext -import-path crates/proto/proto -proto greeter.proto \
  -d '{"name":"world"}' localhost:3001 greeter.Greeter/SayHello
```
