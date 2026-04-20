//! Generated Rust bindings for the workspace's gRPC + REST services.
//!
//! `build.rs` compiles every `.proto` under `proto/` via `tonic-prost-build`
//! (gRPC) and `tonic-rest-build` (REST transcoding from `google.api.http`
//! annotations). The `googleapis/googleapis` submodule under
//! `third_party/googleapis/` is only used as a protoc include path so
//! `import "google/api/annotations.proto"` resolves.

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("file_descriptor_set");

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

#[cfg(test)]
mod tests {
    use super::greeter::HelloRequest;
    use prost_validate::Validator;

    #[test]
    fn empty_name_fails_validation() {
        let req = HelloRequest {
            name: String::new(),
        };
        let err = req
            .validate()
            .expect_err("empty name should violate min_len: 1");
        assert!(
            err.to_string().to_lowercase().contains("length"),
            "expected length-related error, got: {err}"
        );
    }

    #[test]
    fn non_empty_name_passes_validation() {
        let req = HelloRequest {
            name: "world".into(),
        };
        req.validate().expect("non-empty name should pass");
    }
}
