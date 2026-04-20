fn main() -> Result<(), Box<dyn std::error::Error>> {
    let includes = &[
        "proto".to_string(),
        "third_party/googleapis".to_string(),
    ];

    let proto_files: &[String] = &[
        "proto/greeter.proto".to_string(),
        // Street View Publish API — the "photos" gRPC API exposed in googleapis.
        "third_party/googleapis/google/streetview/publish/v1/streetview_publish.proto".to_string(),
    ];

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos(proto_files, includes)?;

    println!("cargo:rerun-if-changed=proto");
    println!("cargo:rerun-if-changed=third_party/googleapis");
    Ok(())
}
