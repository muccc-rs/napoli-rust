use std::io::Result;
fn main() -> Result<()> {
    // prost_build::compile_protos(&["proto/napoli.proto"], &["proto/"])?;
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["proto/models.proto", "proto/comms.proto"], &["proto/"])?;
    Ok(())
}