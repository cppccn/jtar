//! # `JTAR`
//!
//! _JTAR is a dumb archive format that appears to be a subset of JSON (which is handy)_

use rayon::prelude::*;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
enum Mode {
    EncodeEverything,
    EncodeOnlyBinaries,
    SkipNonUtf8Files,
}

// TODO: pass it as args
const MODE: Mode = Mode::SkipNonUtf8Files;

#[derive(serde::Serialize)]
struct SystemTime {}

#[derive(serde::Serialize)]
struct FileType {}

#[derive(serde::Serialize)]
struct Permissions {}

#[derive(serde::Serialize)]
struct Metadata {
    file_type: FileType,
    is_dir: bool,
    is_file: bool,
    permissions: Permissions,
    modified: SystemTime,
    accessed: SystemTime,
    created: SystemTime,
}

#[derive(serde::Serialize)]
enum Content {
    Base64(String),
    Utf8(String),
    Skiped,
    Folder,
}

#[derive(serde::Serialize)]
struct Entry {
    // metadata: Metadata, // FIXME
    path: String,
    content: Content,
}

fn utf8(path: &Path) -> Result<Content, ()> {
    match std::fs::read_to_string(path) {
        Ok(x) => Ok(Content::Utf8(x)),
        Err(_) => Err(()),
    }
}

fn base64(path: &Path) -> Content {
    Content::Base64(base64::encode(
        std::fs::read(path).expect("Failed to read file"),
    ))
}

fn entry(path: &Path) -> Entry {
    let metadata = path.metadata().expect("Failed to read metadata");
    Entry {
        path: path.display().to_string(),
        content: if metadata.is_file() {
            match MODE {
                Mode::EncodeEverything => base64(path),
                Mode::EncodeOnlyBinaries => utf8(path).unwrap_or_else(|_| base64(path)),
                Mode::SkipNonUtf8Files => utf8(path).unwrap_or(Content::Skiped),
            }
        } else {
            Content::Folder
        },
    }
}

pub fn compress(dir: &Path) -> String {
    let paths: Vec<PathBuf> = glob::glob(dir.to_str().unwrap())
        .expect("Failed to read glob pattern")
        .map(|x| x.unwrap())
        .collect();
    let entries: Vec<Entry> = paths.par_iter().map(|x| entry(x)).collect();
    serde_json::to_string(&entries).expect("Failed to encode JSON")
}

pub fn extract(_dir: &Path) {
    unimplemented!();
}
