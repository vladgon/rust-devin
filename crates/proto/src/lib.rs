//! Generated Rust bindings for the workspace's gRPC + REST services.
//!
//! Protobuf definitions live under `proto/` (workspace-owned) and
//! `third_party/googleapis/` (git submodule). `build.rs` compiles them via
//! `tonic-prost-build` (gRPC) and `tonic-rest-build` (REST transcoding from
//! `google.api.http` annotations).

/// Local `Greeter` example service.
pub mod greeter {
    tonic::include_proto!("greeter");
}

/// Google API protobufs re-exported by package path.
pub mod google {
    pub mod api {
        tonic::include_proto!("google.api");
    }
    pub mod longrunning {
        tonic::include_proto!("google.longrunning");
    }
    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
    pub mod r#type {
        tonic::include_proto!("google.r#type");
    }
    pub mod streetview {
        pub mod publish {
            pub mod v1 {
                tonic::include_proto!("google.streetview.publish.v1");
            }
        }
    }
}

/// REST routers generated from `google.api.http` annotations.
///
/// Contains per-service `*_rest_router(Arc<S>) -> axum::Router` functions for
/// every service in our proto tree that has HTTP bindings.
pub mod rest {
    include!(concat!(env!("OUT_DIR"), "/rest_routes.rs"));
}
