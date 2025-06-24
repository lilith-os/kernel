use std::env;
use std::path::PathBuf;
use anyhow::Result;

fn main() -> Result<()> {
    let arch = env::var("CARGO_CFG_TARGET_ARCH")?;
    let dir = env::var("CARGO_MANIFEST_DIR")?;
    let linker_file = PathBuf::from(&dir).join(format!("linker-{arch}.ld"));
    let linker_file = linker_file.to_str().unwrap();

    // Tell cargo to pass the linker script to the linker...
    println!("cargo:rustc-link-arg=-T{linker_file}");
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed={linker_file}");
    Ok(())
}