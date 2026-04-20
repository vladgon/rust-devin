//! Generated Rust bindings for the workspace's gRPC + REST services.
//!
//! `build.rs` compiles every `.proto` under `proto/` via `tonic-prost-build`
//! (gRPC) and `tonic-rest-build` (REST transcoding from `google.api.http`
//! annotations). The `googleapis/googleapis` submodule under
//! `third_party/googleapis/` is only used as a protoc include path so
//! `import "google/api/annotations.proto"` resolves.

/// Local `Greeter` example service.
pub mod greeter {
    tonic::include_proto!("greeter");
}

/// REST routers generated from `google.api.http` annotations.
///
/// Contains per-service `*_rest_router(Arc<S>) -> axum::Router` functions for
/// every service in our proto tree that has HTTP bindings.
pub mod rest {
    include!(concat!(env!("OUT_DIR"), "/rest_routes.rs"));
}
