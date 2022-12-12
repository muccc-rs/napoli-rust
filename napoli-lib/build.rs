use std::{env, path::PathBuf};
use std::io::Result;

fn main() -> Result<()> {
    // prost_build::compile_protos(&["proto/napoli.proto"], &["proto/"])?;
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("napoli_descriptor.bin"))
        .compile(&["proto/models.proto", "proto/comms.proto"], &["proto/"])?;
    Ok(())
}