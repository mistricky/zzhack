use pulldown_cmark::{html, Options, Parser};
use std::borrow::Cow;

pub trait MarkdownFilter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str>;
}

#[derive(Default)]
struct FrontmatterFilter;

impl MarkdownFilter for FrontmatterFilter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        match find_frontmatter_body_start(input) {
            Some(start) => Cow::Owned(input[start..].to_string()),
            None => Cow::Borrowed(input),
        }
    }
}

pub struct MarkdownRenderer {
    filters: Vec<Box<dyn MarkdownFilter>>,
    options: Options,
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);

        Self {
            filters: vec![Box::new(FrontmatterFilter::default())],
            options,
        }
    }

    pub fn render(&self, source: &str) -> String {
        let mut processed: Cow<'_, str> = Cow::Borrowed(source);
        for filter in &self.filters {
            let applied = filter.apply(processed.as_ref());
            processed = match applied {
                Cow::Borrowed(_) => processed,
                Cow::Owned(s) => Cow::Owned(s),
            };
        }

        let parser = Parser::new_ext(processed.as_ref(), self.options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        Self::new()
    }
}

fn find_frontmatter_body_start(source: &str) -> Option<usize> {
    let mut lines = source.split_inclusive('\n');
    let first_line = lines.next()?;
    let delimiter = match first_line.trim_end_matches(['\r', '\n']) {
        "--" | "---" => first_line.trim_end_matches(['\r', '\n']),
        _ => return None,
    };

    let mut offset = first_line.len();
    for line in lines {
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed == delimiter {
            return Some(offset + line.len());
        }
        offset += line.len();
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::markdown_renderer::MarkdownRenderer;

    #[test]
    fn strips_double_dash_frontmatter() {
        let src = "--\ntitle: Example\n--\n\n# Heading";
        let html = MarkdownRenderer::new().render(src);
        assert!(!html.contains("title: Example"));
        assert!(html.contains("<h1>Heading</h1>"));
    }

    #[test]
    fn strips_triple_dash_frontmatter() {
        let src = "---\ntitle: Example\n---\nContent";
        let html = MarkdownRenderer::new().render(src);
        assert!(!html.contains("title: Example"));
        assert!(html.contains("<p>Content</p>"));
    }

    #[test]
    fn leaves_regular_content() {
        let src = "# No Frontmatter";
        let html = MarkdownRenderer::new().render(src);
        assert!(html.contains("<h1>No Frontmatter</h1>"));
    }
}
