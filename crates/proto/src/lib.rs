//! Generated Rust bindings for the workspace's gRPC services.
//!
//! Protobuf definitions live under `proto/` and are compiled by `build.rs`
//! via `tonic-build`. Each `package` in a `.proto` file becomes a module here.

pub mod greeter {
    tonic::include_proto!("greeter");
}
