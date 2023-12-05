fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = &[
        "protos/test/test.proto",
        "protos/sys/fhs.proto"
    ];
    // compile protocol buffer using protoc
    tonic_build::configure()
        .build_server(true)
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
            protos,
            &["protos"],
        )?;
    Ok(())
}