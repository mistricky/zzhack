use humantime::format_rfc3339;
use serde::Serialize;
use std::{fs, io, path::Path, time::SystemTime};

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
    pub children_count: Option<usize>,
    pub children: Option<Vec<Entry>>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_post: bool,
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
    let extension = path
        .extension()
        .and_then(|os| os.to_str())
        .map(ToOwned::to_owned);
    let size = if matches!(kind, EntryKind::File) {
        Some(meta.len())
    } else {
        None
    };

    let mut is_post = false;
    let children = if matches!(kind, EntryKind::Directory) && !file_type.is_symlink() {
        let mut entries = Vec::new();
        for child in fs::read_dir(path)? {
            let child = child?;
            let child_path = child.path();
            entries.push(build_entry(&child_path, root)?);
        }
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        let count = entries.len();
        let contains_index = entries.iter().any(|entry| {
            matches!(entry.kind, EntryKind::File) && entry.name.eq_ignore_ascii_case("index.md")
        });
        if contains_index {
            is_post = true;
        }
        Some((entries, count))
    } else {
        None
    };

    let (children, children_count) = match children {
        Some((entries, count)) => (Some(entries), Some(count)),
        None => (None, None),
    };

    let (title, description) = if matches!(kind, EntryKind::File)
        && extension
            .as_deref()
            .map(|ext| ext.eq_ignore_ascii_case("md"))
            == Some(true)
    {
        markdown_front_matter(path)?
    } else {
        (None, None)
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
        children_count,
        children,
        title,
        description,
        is_post,
    })
}

fn format_system_time(ts: SystemTime) -> String {
    format_rfc3339(ts).to_string()
}

fn markdown_front_matter(path: &Path) -> io::Result<(Option<String>, Option<String>)> {
    let content = fs::read_to_string(path)?;
    Ok(parse_front_matter(&content))
}

fn parse_front_matter(content: &str) -> (Option<String>, Option<String>) {
    let mut lines = content.lines().peekable();

    // Skip leading empty lines before checking for the front matter fence.
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            continue;
        }

        if line.trim() != "--" {
            return (None, None);
        }

        let mut title = None;
        let mut description = None;

        for line in lines.by_ref() {
            let trimmed = line.trim();

            if trimmed == "--" {
                break;
            }

            if let Some(value) = trimmed.strip_prefix("title:") {
                title = Some(value.trim().to_string());
            } else if let Some(value) = trimmed.strip_prefix("description:") {
                description = Some(value.trim().to_string());
            }
        }

        return (title, description);
    }

    (None, None)
}
