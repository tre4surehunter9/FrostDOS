// src/filesystem.rs

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use spin::Mutex;
use lazy_static::lazy_static;
use crate::fdfs::Fdfs;

lazy_static! {
    /// The global FDFS instance.
    pub static ref FS: Mutex<Fdfs> = Mutex::new(Fdfs::new());

    /// The current working directory as an absolute path.
    pub static ref CWD: Mutex<String> = Mutex::new("/".to_string());
}

/// Resolve a path relative to the CWD into an absolute path.
/// If the path starts with '/' it's already absolute.
pub fn resolve_path(path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else {
        let cwd = CWD.lock().clone();
        if cwd == "/" {
            alloc::format!("/{}", path)
        } else {
            alloc::format!("{}/{}", cwd, path)
        }
    }
}

pub fn list_dir(path: &str) -> Vec<(String, bool)> {
    FS.lock().list_dir(path).unwrap_or_default()
}

pub fn read_file(path: &str) -> Result<String, &'static str> {
    FS.lock().read_file(path)
}

pub fn write_file(path: &str, contents: &str) -> Result<(), &'static str> {
    FS.lock().write_file(path, contents)
}

pub fn make_dir(path: &str) -> Result<(), &'static str> {
    FS.lock().make_dir(path)
}

pub fn remove(path: &str) -> Result<(), &'static str> {
    FS.lock().remove(path)
}

pub fn is_dir(path: &str) -> bool {
    FS.lock().is_dir(path)
}
