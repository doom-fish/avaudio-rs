#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn artifacts_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-artifacts");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn make_test_audio(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if path.exists() {
        fs::remove_file(path)?;
    }
    let status = Command::new("/usr/bin/say")
        .args([
            "-o",
            path.to_str().ok_or("non-UTF-8 artifact path")?,
            "hello",
        ])
        .status()?;
    if !status.success() {
        return Err(format!("say failed with status {status}").into());
    }
    Ok(())
}
