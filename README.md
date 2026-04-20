# rust-devin

A Cargo workspace. All external dependency versions are declared in the root
[`Cargo.toml`](./Cargo.toml) under `[workspace.dependencies]`, and member
crates reference them with `dep = { workspace = true }`.

## Crates

| Crate | Kind | Purpose |
| --- | --- | --- |
| [`crates/common`](./crates/common) | library | Shared types, error enum, config, tracing setup. |
| [`crates/proto`](./crates/proto) | library | Protobuf definitions + generated tonic-grpc bindings **and** tonic-rest REST handlers (from `google.api.http` annotations). `build.rs` globs every `proto/**/*.proto` in the crate and keeps `googleapis/googleapis` (git submodule at `crates/proto/third_party/googleapis`) on the protoc include path so `google/api/annotations.proto` resolves. |
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

  `proto/build.rs` only compiles workspace-owned protos under `proto/**/*.proto`,
  but still needs the googleapis submodule on disk so `protoc` can resolve
  `import "google/api/annotations.proto"`. No googleapis protos are generated
  into the crate.

## Build & test

```bash
cargo build
cargo test
```

## Pre-commit hook

The repo ships a [`.pre-commit-config.yaml`](./.pre-commit-config.yaml) with a
`cargo fmt --all` hook that runs before every commit. To wire it up:

```bash
pip install pre-commit  # or `brew install pre-commit` / `pipx install pre-commit`
pre-commit install
```

`pre-commit install` is idempotent; run it once per clone.

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
