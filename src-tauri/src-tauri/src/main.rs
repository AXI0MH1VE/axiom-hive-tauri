// url: https://github.com/tauri-apps/tauri/blob/v2/examples/main.rs
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::{Command, Stdio};
use std::fs;
use std::env;
use sha2::{Digest, Sha256};
use tauri::{Manager, Window};

// See: https://docs.rs/sha2/latest/sha2
// See: https://docs.rs/tauri/latest/tauri

const TRUSTED_HASH: &str = include_str!("trusted_sidecar.sha256");

fn compute_sha256<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

fn verify_sidecar(sidecar_path: &str) -> bool {
    if let Ok(hash) = compute_sha256(sidecar_path) {
        hash.trim() == TRUSTED_HASH.trim()
    } else {
        false
    }
}

#[tauri::command]
fn run_sidecar(input: String) -> Result<String, String> {
    let sidecar_path = if cfg!(windows) {
        "sidecar/dist/main.exe"
    } else {
        "sidecar/dist/main"
    };

    if !verify_sidecar(sidecar_path) {
        return Err("Sidecar integrity check failed".to_string());
    }

    let mut child = Command::new(sidecar_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    use std::io::Write;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input.as_bytes())
            .map_err(|e| e.to_string())?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| e.to_string())?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_sidecar])
        .run(tauri::generate_context!())
        .expect("error while running Axiom Hive");
}
