//
// build.rs
// Copyright (C) 2024 imotai <codego.me@gmail.com>
//

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");
    tonic_build::configure()
        .build_server(true)
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .compile_with_config(config, &["thirdparty/tei/proto/tei.proto"], &["proto"])?;
    Ok(())
}
