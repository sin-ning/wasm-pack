//! Building a Rust crate into a `.wasm` binary.

use emoji;
use error::Error;
use progressbar::Step;
use rustc_version::{version, Version};
use std::path::Path;
use std::process::Command;
use PBAR;

pub fn check_rustc_version(step: &Step) -> Result<&str, Error> {
    let local_version = version()?;
    let minimum_version = Version::parse("1.30.0")?;
    if local_version < minimum_version {
        Error::rustc("Your rustc version must be greater than 1.30.0.")
    }
    Ok(&local_version.to_string())
}

/// Ensure that `rustup` has the `wasm32-unknown-unknown` target installed for
/// the `nightly` toolchain.
pub fn rustup_add_wasm_target(step: &Step) -> Result<(), Error> {
    let msg = format!("{}Adding WASM target...", emoji::TARGET);
    PBAR.step(step, &msg);
    ensure_nightly()?;
    let output = Command::new("rustup")
        .arg("target")
        .arg("add")
        .arg("wasm32-unknown-unknown")
        .arg("--toolchain")
        .arg("nightly")
        .output()?;
    if !output.status.success() {
        let s = String::from_utf8_lossy(&output.stderr);
        Error::cli("Adding the wasm32-unknown-unknown target failed", s)
    } else {
        Ok(())
    }
}

/// Ensure that the `nightly` toolchain is installed in `rustup`.
fn ensure_nightly() -> Result<(), Error> {
    let nightly_check = Command::new("rustc").arg("+nightly").arg("-V").output()?;
    if !nightly_check.status.success() {
        let res = Command::new("rustup")
            .arg("toolchain")
            .arg("install")
            .arg("nightly")
            .output()?;
        if !res.status.success() {
            let s = String::from_utf8_lossy(&res.stderr);
            return Error::cli("Adding the nightly toolchain failed", s);
        }
    }
    Ok(())
}

/// Run `cargo build` with the `nightly` toolchain and targetting
/// `wasm32-unknown-unknown`.
pub fn cargo_build_wasm(path: &Path, debug: bool, step: &Step) -> Result<(), Error> {
    let msg = format!("{}Compiling to WASM...", emoji::CYCLONE);
    PBAR.step(step, &msg);
    let output = {
        let mut cmd = Command::new("cargo");
        cmd.current_dir(path)
            .arg("+nightly")
            .arg("build")
            .arg("--lib");
        if !debug {
            cmd.arg("--release");
        }
        cmd.arg("--target").arg("wasm32-unknown-unknown");
        cmd.output()?
    };

    if !output.status.success() {
        let s = String::from_utf8_lossy(&output.stderr);
        Error::cli("Compilation of your program failed", s)
    } else {
        Ok(())
    }
}

/// Run `cargo build --tests` with the `nightly` toolchain and targetting
/// `wasm32-unknown-unknown`.
pub fn cargo_build_wasm_tests(path: &Path, debug: bool) -> Result<(), Error> {
    let output = {
        let mut cmd = Command::new("cargo");
        cmd.current_dir(path)
            .arg("+nightly")
            .arg("build")
            .arg("--tests");
        if !debug {
            cmd.arg("--release");
        }
        cmd.arg("--target").arg("wasm32-unknown-unknown");
        cmd.output()?
    };

    if !output.status.success() {
        let s = String::from_utf8_lossy(&output.stderr);
        Error::cli("Compilation of your program failed", s)
    } else {
        Ok(())
    }
}
