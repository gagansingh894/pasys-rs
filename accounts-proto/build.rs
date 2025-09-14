use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    // Root of proto root
    let proto_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
        .join("proto");

    // Paths to proto files
    let proto_file = proto_root.join("pasys/services/accounts/v1/accounts.proto");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("accounts_v1_descriptor.bin"))
        .out_dir(&out_dir)
        .compile(
            &[proto_file.to_str().unwrap()],
            &[proto_root.to_str().unwrap()],
        )?;

    println!("cargo:rerun-if-changed={}", proto_file.display());
    println!("cargo:rerun-if-changed={}", proto_root.display());

    Ok(())
}
