use std::path::Path;

/// Return the `COVER.png` path in the same directory as the provided path.
#[allow(dead_code)]
pub fn get_cover_path(path: &str) -> String {
    let parent = Path::new(path).parent();

    match parent {
        Some(dir) if !dir.as_os_str().is_empty() => {
            dir.join("COVER.png").to_string_lossy().into_owned()
        }
        _ => "COVER.png".to_string(),
    }
}

/// Prefix the given relative path with the data root.
#[allow(dead_code)]
pub fn parse_data_url(path: &str) -> String {
    format!("/data/{path}")
}

#[cfg(test)]
mod tests {
    use super::get_cover_path;
    use super::parse_data_url;

    #[test]
    fn returns_cover_at_root_for_top_level_files() {
        assert_eq!(get_cover_path("a.js"), "COVER.png");
    }

    #[test]
    fn returns_cover_in_same_directory() {
        assert_eq!(get_cover_path("foo/xxx"), "foo/COVER.png");
        assert_eq!(get_cover_path("foo/bar/baz.md"), "foo/bar/COVER.png");
    }

    #[test]
    fn prefixes_data_root() {
        assert_eq!(parse_data_url("links/index.md"), "/data/links/index.md");
        assert_eq!(parse_data_url("file.txt"), "/data/file.txt");
    }
}
