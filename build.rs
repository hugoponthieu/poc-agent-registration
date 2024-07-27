use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let _ = tonic_build::compile_protos("proto/registration.proto");
    let _ = tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("registration_descriptor.bin"))
        .compile(&["proto/registration.proto"], &["proto"]);
    Ok(())
}
