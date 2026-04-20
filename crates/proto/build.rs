use std::env;
use std::path::PathBuf;

use tonic_rest_build::{dump_file_descriptor_set, generate, RestCodegenConfig};

/// All proto files compiled for gRPC (messages + service traits).
const GRPC_PROTO_FILES: &[&str] = &[
    "proto/greeter.proto",
    // Street View Publish API — the "photos" gRPC API in googleapis.
    "third_party/googleapis/google/streetview/publish/v1/streetview_publish.proto",
];

/// Subset compiled for REST transcoding via `tonic-rest`. Excludes APIs whose
/// `google.api.http` bindings use features `tonic-rest` 0.1 doesn't yet
/// support (e.g. partial-body selectors like `body: "photo"` in Street View
/// Publish).
const REST_PROTO_FILES: &[&str] = &["proto/greeter.proto"];

const PROTO_INCLUDES: &[&str] = &["proto", "third_party/googleapis"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir: PathBuf = env::var_os("OUT_DIR").expect("OUT_DIR not set").into();
    let descriptor_path = out_dir.join("file_descriptor_set.bin");
    let descriptor_path_str = descriptor_path
        .to_str()
        .expect("OUT_DIR contains non-UTF8 bytes");

    // Phase 1: descriptor set for the REST-eligible subset, consumed by
    // `tonic-rest` codegen below.
    let rest_descriptor_bytes =
        dump_file_descriptor_set(REST_PROTO_FILES, PROTO_INCLUDES, descriptor_path_str);

    // Phase 2: tonic + prost codegen for all protos. Serde derives are
    // applied only to the `greeter` package (our REST-eligible types),
    // leaving Street View Publish and the other Google API packages alone.
    let mut prost_config = prost_build::Config::new();
    prost_config.message_attribute(
        ".greeter",
        "#[derive(serde::Serialize, serde::Deserialize)] \
         #[serde(rename_all = \"camelCase\")]",
    );
    prost_config.enum_attribute(
        ".greeter",
        "#[derive(serde::Serialize, serde::Deserialize)]",
    );

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_with_config(prost_config, GRPC_PROTO_FILES, PROTO_INCLUDES)?;

    // Phase 3: REST + SSE codegen from `google.api.http` annotations.
    let rest_config = RestCodegenConfig::new();
    let code = generate(&rest_descriptor_bytes, &rest_config)?;
    std::fs::write(out_dir.join("rest_routes.rs"), code)?;

    println!("cargo:rerun-if-changed=proto");
    println!("cargo:rerun-if-changed=third_party/googleapis");
    Ok(())
}
