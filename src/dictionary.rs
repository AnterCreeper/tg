#![allow(dead_code)]

use std::path::{Path, PathBuf};

const MASK: u8 = 0x5a;

fn decode(encoded: &[u8]) -> String {
    let bytes: Vec<u8> = encoded.iter().map(|byte| byte ^ MASK).collect();
    String::from_utf8(bytes).expect("dictionary entry must be valid UTF-8")
}

fn decode_bytes(encoded: &[u8]) -> Vec<u8> {
    encoded.iter().map(|byte| byte ^ MASK).collect()
}

#[cfg(target_os = "macos")]
pub(crate) fn desktop_app_process() -> String {
    decode(&[13, 63, 25, 50, 59, 46])
}

#[cfg(target_os = "linux")]
pub(crate) fn desktop_app_process() -> String {
    decode(&[45, 63, 57, 50, 59, 46])
}

pub(crate) fn desktop_app_name() -> String {
    desktop_app_process()
}

pub(crate) fn desktop_app_localized_name() -> String {
    decode(&[191, 228, 244, 190, 229, 251])
}

pub(crate) fn container_id() -> String {
    decode(&[
        57, 53, 55, 116, 46, 63, 52, 57, 63, 52, 46, 116, 34, 51, 52, 13, 63, 25, 50, 59, 46,
    ])
}

pub(crate) fn account_files_dir() -> String {
    decode(&[34, 45, 63, 57, 50, 59, 46, 5, 60, 51, 54, 63, 41])
}

pub(crate) fn account_id_prefix() -> String {
    decode(&[45, 34, 51, 62, 5])
}

pub(crate) fn sticker_magic() -> Vec<u8> {
    decode_bytes(&[45, 34, 61, 60])
}

pub(crate) fn msg_body_column() -> String {
    decode(&[55, 63, 41, 41, 59, 61, 63, 5, 57, 53, 52, 46, 63, 52, 46])
}

pub(crate) fn msg_compression_marker_column() -> String {
    decode(&[
        13, 25, 30, 24, 5, 25, 14, 5, 55, 63, 41, 41, 59, 61, 63, 5, 57, 53, 52, 46, 63, 52, 46,
    ])
}

pub(crate) fn msg_sender_column() -> String {
    decode(&[40, 63, 59, 54, 5, 41, 63, 52, 62, 63, 40, 5, 51, 62])
}

pub(crate) fn msg_packed_meta_column() -> String {
    decode(&[42, 59, 57, 49, 63, 62, 5, 51, 52, 60, 53, 5, 62, 59, 46, 59])
}

pub(crate) fn real_home_dir() -> Option<PathBuf> {
    if let Ok(sudo_user) = std::env::var("SUDO_USER") {
        if !sudo_user.is_empty() && sudo_user != "root" {
            #[cfg(target_os = "macos")]
            return Some(PathBuf::from("/Users").join(sudo_user));
            #[cfg(target_os = "linux")]
            return Some(PathBuf::from("/home").join(sudo_user));
        }
    }
    std::env::var("HOME").ok().map(PathBuf::from)
}

#[cfg(target_os = "macos")]
pub(crate) fn container_data_dir(home: &Path) -> PathBuf {
    home.join("Library/Containers")
        .join(container_id())
        .join("Data")
}

#[cfg(target_os = "linux")]
pub(crate) fn container_data_dir(home: &Path) -> PathBuf {
    home.to_path_buf()
}

#[cfg(target_os = "macos")]
pub(crate) fn documents_account_files_dir(home: &Path) -> PathBuf {
    container_data_dir(home)
        .join("Documents")
        .join(account_files_dir())
}

#[cfg(target_os = "linux")]
pub(crate) fn documents_account_files_dir(home: &Path) -> PathBuf {
    xdg_documents_dir(home)
        .unwrap_or_else(|| home.join("Documents"))
        .join(account_files_dir())
}

#[cfg(target_os = "linux")]
fn xdg_documents_dir(home: &Path) -> Option<PathBuf> {
    use std::process::Command;

    let output = Command::new("xdg-user-dir")
        .arg("DOCUMENTS")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let path = String::from_utf8_lossy(&output.stdout);
    let path = path.trim();

    if path.is_empty() || path == home.to_string_lossy().as_ref() {
        return None;
    }

    let path = PathBuf::from(path);
    if path.is_dir() {
        Some(path)
    } else {
        None
    }
}

#[cfg(target_os = "macos")]
pub(crate) fn app_support_dir(home: &Path) -> PathBuf {
    container_data_dir(home)
        .join("Library/Application Support")
        .join(container_id())
}

#[cfg(target_os = "linux")]
pub(crate) fn app_support_dir(home: &Path) -> PathBuf {
    home.join(".local/share").join(desktop_app_process())
}

#[cfg(target_os = "macos")]
pub(crate) fn kvcomm_dir(home: &Path) -> PathBuf {
    container_data_dir(home).join("Documents/app_data/net/kvcomm")
}

#[cfg(target_os = "linux")]
pub(crate) fn kvcomm_dir(_home: &Path) -> PathBuf {
    PathBuf::new()
}
