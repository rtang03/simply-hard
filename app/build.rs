use std::env;

// NOTE:
// https://github.com/hyperium/tonic/blob/master/examples/build.rs
// https://github.com/hyperium/tonic/issues/1331
// https://github.com/protocolbuffers/protobuf/blob/main/docs/implementing_proto3_presence.md

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "./proto/echo.proto";

    match env::var("SKIP_COMPILE_PROTO") {
        Err(_) => {
            println!("*** compiling protocol-buffer ***");

            tonic_build::configure()
                .build_server(true)
                .type_attribute("echo.EchoRequest", "#[derive(Hash)]")
                .type_attribute("echo.EchoResponse", "#[derive(Hash)]")
                .server_mod_attribute("attrs", "#[cfg(feature = \"server\")]")
                .client_mod_attribute("attrs", "#[cfg(feature = \"client\")]")
                .out_dir("./src")
                .protoc_arg("--experimental_allow_proto3_optional")
                .compile(&[proto_file], &["."])
                .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

            println!("cargo:rerun-if-changed={}", proto_file);
        }
        Ok(_) => println!("protocolbuffer compilation skipped"),
    }

    Ok(())
}
