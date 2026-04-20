use std::env;
use std::path::PathBuf;

use tonic_rest_build::{dump_file_descriptor_set, generate, RestCodegenConfig};

/// Directory holding workspace-owned `.proto` files. Every `*.proto` under
/// this path (recursively) is compiled for gRPC **and** REST transcoding.
const PROTO_DIR: &str = "proto";

/// Additional googleapis protos compiled for gRPC only. These are excluded
/// from REST codegen because their `google.api.http` bindings use features
/// `tonic-rest` 0.1 doesn't yet support (e.g. partial-body selectors like
/// `body: "photo"` in Street View Publish).
const EXTRA_GRPC_PROTO_FILES: &[&str] = &[
    // Street View Publish API — the "photos" gRPC API in googleapis.
    "third_party/googleapis/google/streetview/publish/v1/streetview_publish.proto",
];

const PROTO_INCLUDES: &[&str] = &["proto", "third_party/googleapis"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir: PathBuf = env::var_os("OUT_DIR").expect("OUT_DIR not set").into();
    let descriptor_path = out_dir.join("file_descriptor_set.bin");
    let descriptor_path_str = descriptor_path
        .to_str()
        .expect("OUT_DIR contains non-UTF8 bytes");

    // Discover every workspace-owned `.proto` file under `proto/` so new
    // protos are picked up automatically.
    let rest_proto_files: Vec<String> = glob::glob(&format!("{PROTO_DIR}/**/*.proto"))?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|p| {
            p.to_str()
                .expect("proto path contains non-UTF8 bytes")
                .to_owned()
        })
        .collect();
    let rest_proto_refs: Vec<&str> = rest_proto_files.iter().map(String::as_str).collect();

    let grpc_proto_refs: Vec<&str> = rest_proto_refs
        .iter()
        .copied()
        .chain(EXTRA_GRPC_PROTO_FILES.iter().copied())
        .collect();

    // Phase 1: descriptor set for the REST-eligible subset, consumed by
    // `tonic-rest` codegen below.
    let rest_descriptor_bytes =
        dump_file_descriptor_set(&rest_proto_refs, PROTO_INCLUDES, descriptor_path_str);

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
        .compile_with_config(prost_config, &grpc_proto_refs, PROTO_INCLUDES)?;

    // Phase 3: REST + SSE codegen from `google.api.http` annotations.
    let rest_config = RestCodegenConfig::new();
    let code = generate(&rest_descriptor_bytes, &rest_config)?;
    std::fs::write(out_dir.join("rest_routes.rs"), code)?;

    println!("cargo:rerun-if-changed={PROTO_DIR}");
    println!("cargo:rerun-if-changed=third_party/googleapis");
    Ok(())
}
