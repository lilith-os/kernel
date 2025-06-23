use std::env;
use std::fs::{create_dir_all, remove_file};
use std::os::windows::fs::symlink_file;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use anyhow::{Result};

fn main() -> Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let runner_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let limine_dir = PathBuf::from(env::var("LIMINE_PATH")?);

    let iso_dir = out_dir.join("iso_root");
    create_dir_all(&iso_dir)?;

    let limine_conf = iso_dir.join("limine.conf");
    ensure_symlink(runner_dir.join("limine.conf"), &limine_conf)?;

    let boot_dir = iso_dir.join("boot");
    create_dir_all(&boot_dir)?;

    let out_limine_dir = boot_dir.join("limine");
    create_dir_all(&out_limine_dir)?;

    for path in [
        "limine-bios.sys",
        "limine-bios-cd.bin",
        "limine-uefi-cd.bin",
    ] {
        let from = limine_dir.join(path);
        let to = out_limine_dir.join(path);
        ensure_symlink(&from, &to)?;
    }

    let efi_boot_dir = iso_dir.join("EFI/BOOT");
    create_dir_all(&efi_boot_dir)?;

    for efi_file in ["BOOTX64.EFI", "BOOTIA32.EFI"] {
        ensure_symlink(limine_dir.join(efi_file), efi_boot_dir.join(efi_file))?;
    }

    let output_iso = out_dir.join("os.iso");

    let status = Command::new("wsl")
        .arg("xorriso")
        .arg("-as")
        .arg("mkisofs")
        .arg("--follow-links")
        .arg("-b")
        .arg(
            out_limine_dir
                .join("limine-bios-cd.bin")
                .strip_prefix(&iso_dir)
                .unwrap()
                .display()
                .to_string()
                .replace("\\", "/")
        )
        .arg("-no-emul-boot")
        .arg("-boot-load-size")
        .arg("4")
        .arg("-boot-info-table")
        .arg("--efi-boot")
        .arg(
            out_limine_dir
                .join("limine-uefi-cd.bin")
                .strip_prefix(&iso_dir)
                .unwrap()
                .display()
                .to_string()
                .replace("\\", "/")
        )
        .arg("-efi-boot-part")
        .arg("--efi-boot-image")
        .arg("--protective-msdos-label")
        .arg(to_wsl_path(iso_dir.display().to_string().replace("\\", "/")))
        .arg("-o")
        .arg(to_wsl_path(output_iso.display().to_string().replace("\\", "/")))
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()?;
    assert!(status.success());

    let status = std::process::Command::new("limine")
        .arg("bios-install")
        .arg(&output_iso)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()?;
    assert!(status.success());

    let output_iso = output_iso.display();
    println!("cargo:rustc-env=ISO={output_iso}");
    
    Ok(())
}

pub fn ensure_symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> Result<()> {
    match remove_file(&link) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err),
    }?;
    symlink_file(original, link)?;
    Ok(())
}

fn to_wsl_path<P: AsRef<Path>>(path: P) -> String {
    let path = path.as_ref();
    let mut result = String::from("/mnt/");
    let path_str = path.to_string_lossy();

    if let Some(drive_letter) = path_str.chars().next() {
        if drive_letter.is_ascii_alphabetic() {
            result.push(drive_letter.to_ascii_lowercase());
            result.push('/');
            result.push_str(&path_str[2..].replace("\\", "/"));
        }
    }

    result
}
