use humantime::format_rfc3339;
use serde::Serialize;
use std::{fs, path::Path, time::SystemTime};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryKind {
    Directory,
    File,
    Symlink,
    Other,
}

#[derive(Debug, Serialize)]
pub struct Entry {
    pub name: String,
    pub path: String,
    pub kind: EntryKind,
    pub extension: Option<String>,
    pub size: Option<u64>,
    pub modified: Option<String>,
    pub read_only: bool,
    pub children_count: Option<usize>,
    pub children: Option<Vec<Entry>>,
}

pub fn generate_metadata(root: &Path) -> std::io::Result<Entry> {
    let canonical_root = if root.exists() {
        root.to_path_buf()
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("metadata root not found: {}", root.display()),
        ));
    };
    build_entry(&canonical_root, &canonical_root)
}

pub fn generate_metadata_json(root: &Path) -> std::io::Result<String> {
    let entry = generate_metadata(root)?;
    let json = serde_json::to_string_pretty(&entry)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
    Ok(json)
}

fn build_entry(path: &Path, root: &Path) -> std::io::Result<Entry> {
    let meta = fs::symlink_metadata(path)?;
    let file_type = meta.file_type();
    let kind = if file_type.is_dir() {
        EntryKind::Directory
    } else if file_type.is_file() {
        EntryKind::File
    } else if file_type.is_symlink() {
        EntryKind::Symlink
    } else {
        EntryKind::Other
    };

    let name = path
        .file_name()
        .map(|os| os.to_string_lossy().into_owned())
        .unwrap_or_else(|| String::from("."));

    let relative = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned();

    let modified = meta.modified().ok().map(format_system_time);
    let read_only = meta.permissions().readonly();
    let extension = path
        .extension()
        .and_then(|os| os.to_str())
        .map(ToOwned::to_owned);
    let size = if matches!(kind, EntryKind::File) {
        Some(meta.len())
    } else {
        None
    };

    let children = if matches!(kind, EntryKind::Directory) && !file_type.is_symlink() {
        let mut entries = Vec::new();
        for child in fs::read_dir(path)? {
            let child = child?;
            let child_path = child.path();
            entries.push(build_entry(&child_path, root)?);
        }
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        let count = entries.len();
        Some((entries, count))
    } else {
        None
    };

    let (children, children_count) = match children {
        Some((entries, count)) => (Some(entries), Some(count)),
        None => (None, None),
    };

    Ok(Entry {
        name,
        path: if relative.is_empty() {
            ".".into()
        } else {
            relative
        },
        kind,
        extension,
        size,
        modified,
        read_only,
        children_count,
        children,
    })
}

fn format_system_time(ts: SystemTime) -> String {
    format_rfc3339(ts).to_string()
}
