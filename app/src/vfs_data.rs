use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct VfsNode {
    pub name: String,
    pub path: String,
    pub kind: VfsKind,
    pub extension: Option<String>,
    pub size: Option<u64>,
    pub modified: Option<String>,
    pub read_only: bool,
    pub children_count: Option<usize>,
    pub children: Option<Vec<VfsNode>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VfsKind {
    Directory,
    File,
    Symlink,
    Other,
}

pub fn load_vfs() -> VfsNode {
    let raw = include_str!("vfs.json");
    serde_json::from_str(raw).expect("failed to parse bundled vfs metadata")
}

pub fn resolve_path(current: &[String], input: &str) -> Vec<String> {
    let mut parts: Vec<String> = if input.starts_with('/') {
        Vec::new()
    } else {
        current.to_vec()
    };

    for part in input.split('/') {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            parts.pop();
            continue;
        }
        parts.push(part.to_string());
    }
    parts
}

pub fn format_path(parts: &[String]) -> String {
    if parts.is_empty() {
        "/".into()
    } else {
        format!("/{}", parts.join("/"))
    }
}

pub fn find_node<'a>(root: &'a VfsNode, path: &[String]) -> Option<&'a VfsNode> {
    let mut node = root;
    for segment in path {
        node = node
            .children
            .as_ref()?
            .iter()
            .find(|child| child.name == *segment)?;
    }
    Some(node)
}

#[allow(dead_code)]
pub fn list_children(node: &VfsNode) -> Option<Vec<String>> {
    node.children.as_ref().map(|children| {
        let mut names: Vec<String> = children
            .iter()
            .map(|child| {
                if child.kind == VfsKind::Directory {
                    format!("{}/", child.name)
                } else {
                    child.name.clone()
                }
            })
            .collect();
        names.sort();
        names
    })
}

pub fn node_summary(node: &VfsNode) -> String {
    let kind = match node.kind {
        VfsKind::Directory => "dir",
        VfsKind::File => "file",
        VfsKind::Symlink => "symlink",
        VfsKind::Other => "other",
    };
    let size = node
        .size
        .map(|s| format!("{s} bytes"))
        .unwrap_or_else(|| "-".into());
    let modified = node.modified.clone().unwrap_or_else(|| "-".into());
    let ro = if node.read_only { "ro" } else { "rw" };

    format!(
        "kind={kind} size={size} perms={ro} modified={modified} ext={}",
        node.extension.clone().unwrap_or_else(|| "-".into())
    )
}

pub fn du_bytes(node: &VfsNode) -> u64 {
    let mut total = node.size.unwrap_or(0);
    if let Some(children) = &node.children {
        for child in children {
            total += du_bytes(child);
        }
    }
    total
}
