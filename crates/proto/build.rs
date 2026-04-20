fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = &["proto/greeter.proto"];
    let includes = &["proto"];

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos(proto_files, includes)?;

    for file in proto_files {
        println!("cargo:rerun-if-changed={file}");
    }
    Ok(())
}
