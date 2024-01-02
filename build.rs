fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = &[
        "protos/fhs.proto",
        "protos/recipe.proto",
        "protos/service.proto",
    ];
    // compile protocol buffer using protoc
    tonic_build::configure()
        .build_server(true)
        .proto_path("protos/")
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_client(true)
        .build_server(true)
        .compile(protos, &["protos/"])?;
    Ok(())
}
