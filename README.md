# rust-devin

A Cargo workspace. All external dependency versions are declared in the root
[`Cargo.toml`](./Cargo.toml) under `[workspace.dependencies]`, and member
crates reference them with `dep = { workspace = true }`.

## Crates

| Crate | Kind | Purpose |
| --- | --- | --- |
| [`crates/common`](./crates/common) | library | Shared types, error enum, config, tracing setup. |
| [`crates/proto`](./crates/proto) | library | Protobuf definitions + generated tonic-grpc bindings **and** tonic-rest REST handlers (from `google.api.http` annotations). Includes `googleapis/googleapis` as a git submodule under `crates/proto/third_party/googleapis`. |
| [`crates/web`](./crates/web) | binary | Axum HTTP server (with REST-transcoded `Greeter.SayHello`) + tonic-grpc Greeter server on port `+1`. |

## Prerequisites

- Rust (stable, edition 2021+)
- `protoc` (protobuf compiler) — required at build time by `tonic-prost-build`.
  - Ubuntu/Debian: `sudo apt-get install -y protobuf-compiler`
  - macOS: `brew install protobuf`
- Submodules populated:

  ```bash
  git clone --recurse-submodules https://github.com/vladgon/rust-devin
  # or, in an existing clone:
  git submodule update --init --recursive
  ```

  The `proto` crate compiles a subset of the `googleapis/googleapis` protos;
  the submodule lives at `crates/proto/third_party/googleapis` and is currently
  wired up to generate bindings for the Street View Publish API (the "photos"
  gRPC API in googleapis — Google Photos itself is REST-only and has no
  official proto definitions).

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
