fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = &[
        "protos/fhs.proto",
        "protos/instruction.proto",
        "protos/recipe.proto",
        "protos/service.proto",
    ];
    // compile protocol buffer using protoc
    tonic_build::configure()
        .build_server(true)
        .proto_path("protos/")
        .protoc_arg("--experimental_allow_proto3_optional")
        // .message_attribute("./protos/", "#[derive(Iterable)]")
        .enum_attribute("./protos/db.proto", "#(derive(Iden))")
        .compile(protos, &["protos/"])?;
    Ok(())
}
