use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = &[
        "protos/fhs.proto",
        "protos/recipe.proto",
        "protos/service.proto",
        "protos/gpio.proto",
        "protos/drink_controller.proto",
    ];
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // compile protocol buffer using protoc
    tonic_build::configure()
        .proto_path("protos/")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(protos, &["protos/"])?;

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("store_descriptor.bin"))
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &[
                "protos/server.proto",
                "protos/drink_controller_server.proto",
            ],
            &["protos/"],
        )?;
    Ok(())
}
