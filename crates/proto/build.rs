use std::env;
use std::path::PathBuf;

use tonic_rest_build::{
    configure_prost_serde, dump_file_descriptor_set, generate, RestCodegenConfig,
};

/// Directory holding workspace-owned `.proto` files. Every `*.proto` under
/// this path (recursively) is compiled for gRPC **and** REST transcoding.
const PROTO_DIR: &str = "proto";

/// Include paths passed to `protoc`.
///
/// - `third_party/googleapis` resolves `import "google/api/annotations.proto"`
///   for `google.api.http` bindings.
/// - `third_party/protoc-gen-validate` resolves
///   `import "validate/validate.proto"` for `prost-validate` /
///   protoc-gen-validate field rules (vendored upstream at
///   `third_party/protoc-gen-validate/validate/validate.proto`).
///
/// We don't compile any of these vendored protos into Rust ourselves; they're
/// only used by `protoc` to resolve imports and by build-time tooling.
const PROTO_INCLUDES: &[&str] = &[
    "proto",
    "third_party/googleapis",
    "third_party/protoc-gen-validate",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir: PathBuf = env::var_os("OUT_DIR").expect("OUT_DIR not set").into();
    let descriptor_path = out_dir.join("file_descriptor_set.bin");
    let descriptor_path_str = descriptor_path
        .to_str()
        .expect("OUT_DIR contains non-UTF8 bytes");

    // Discover every workspace-owned `.proto` file under `proto/` so new
    // protos are picked up automatically.
    let proto_files: Vec<String> = glob::glob(&format!("{PROTO_DIR}/**/*.proto"))?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|p| {
            p.to_str()
                .expect("proto path contains non-UTF8 bytes")
                .to_owned()
        })
        .collect();
    let proto_refs: Vec<&str> = proto_files.iter().map(String::as_str).collect();

    // Phase 1: descriptor set consumed by `tonic-rest` codegen and by
    // `configure_prost_serde` to auto-wire WKT adapters.
    let descriptor_bytes =
        dump_file_descriptor_set(&proto_refs, PROTO_INCLUDES, descriptor_path_str);

    // Phase 2: tonic + prost codegen. `configure_prost_serde` applies
    // `#[derive(serde::Serialize, serde::Deserialize)]` to every message/enum
    // (path `"."`) and auto-wires `#[serde(with = …)]` adapters for
    // well-known types so `prost_types::{Timestamp, Duration, FieldMask}`
    // round-trip through JSON.
    let mut prost_config = prost_build::Config::new();
    configure_prost_serde(
        &mut prost_config,
        &descriptor_bytes,
        &proto_refs,
        "tonic_rest::serde",
        &[
            (".google.protobuf.Timestamp", "opt_timestamp"),
            (".google.protobuf.Duration", "opt_duration"),
            (".google.protobuf.FieldMask", "opt_field_mask"),
        ],
        &[],
    );

    // Phase 2b: prost-validate — add `#[derive(prost_validate::Validator)]`
    // + per-field rule attributes for every message carrying
    // `(validate.rules)` annotations.
    prost_validate_build::Builder::new().configure(&mut prost_config, &proto_refs, PROTO_INCLUDES)?;

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_with_config(prost_config, &proto_refs, PROTO_INCLUDES)?;

    // Phase 3: REST + SSE codegen from `google.api.http` annotations.
    let rest_config = RestCodegenConfig::new();
    let code = generate(&descriptor_bytes, &rest_config)?;
    std::fs::write(out_dir.join("rest_routes.rs"), code)?;

    println!("cargo:rerun-if-changed={PROTO_DIR}");
    println!("cargo:rerun-if-changed=third_party/googleapis");
    println!("cargo:rerun-if-changed=third_party/protoc-gen-validate");
    Ok(())
}
