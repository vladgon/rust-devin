# rust-devin

A Cargo workspace. All external dependency versions are declared in the root
[`Cargo.toml`](./Cargo.toml) under `[workspace.dependencies]`, and member
crates reference them with `dep = { workspace = true }`.

## Crates

| Crate | Kind | Purpose |
| --- | --- | --- |
| [`crates/common`](./crates/common) | library | Shared types, error enum, config, tracing setup. |
| [`crates/photo`](./crates/photo) | library | Photo decoding, metadata, and thumbnailing (built on `image`). |
| [`crates/web`](./crates/web) | binary | Axum HTTP server exposing `/health` and `/photo/info`. |

## Build & test

```bash
cargo build
cargo test
```

## Run the web server

```bash
cargo run -p web
# then
curl http://localhost:3000/health
curl --data-binary @some.jpg http://localhost:3000/photo/info
```
