// https://github.com/hyperium/tonic/blob/master/examples/build.rs

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let proto_file = "./proto/gupload.proto";
    // tonic_build::configure()
    //     .build_server(true)
    //     .out_dir("./src")
    //     .compile(&[proto_file], &["."])
    //     .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    let proto_file = "./proto/echo.proto";

    tonic_build::configure()
        .build_server(true)
        .type_attribute("echo.EchoRequest", "#[derive(Hash)]")
        .type_attribute("echo.EchoResponse", "#[derive(Hash)]")
        .server_mod_attribute("attrs", "#[cfg(feature = \"server\")]")
        .client_mod_attribute("attrs", "#[cfg(feature = \"client\")]")
        .out_dir("./src")
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", proto_file);

    Ok(())
}
