//! Generated Rust bindings for the workspace's gRPC + REST services.
//!
//! - Our own protos live under `proto/` and are compiled at build time by
//!   `tonic-prost-build` (gRPC) and `tonic-rest-build` (REST transcoding from
//!   `google.api.http` annotations).
//! - Google API types (`google.api.*`, etc.) are sourced from the prebuilt
//!   [`google-api-proto`] crate and re-exported under [`google`] rather than
//!   compiled locally.
//!
//! The `googleapis/googleapis` submodule under `third_party/googleapis/` is
//! kept only so `build.rs` can resolve `import "google/api/annotations.proto"`
//! during proto compilation.

/// Local `Greeter` example service.
pub mod greeter {
    tonic::include_proto!("greeter");
}

/// Google API protobufs, re-exported from the prebuilt
/// [`google-api-proto`](https://crates.io/crates/google-api-proto) crate.
pub use google_api_proto::google;

/// REST routers generated from `google.api.http` annotations.
///
/// Contains per-service `*_rest_router(Arc<S>) -> axum::Router` functions for
/// every service in our proto tree that has HTTP bindings.
pub mod rest {
    include!(concat!(env!("OUT_DIR"), "/rest_routes.rs"));
}
