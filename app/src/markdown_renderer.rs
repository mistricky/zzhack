use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag};
use std::borrow::Cow;
use yew::prelude::*;

use crate::components::markdown_renderer::{
    Blockquote, CodeBlock, Image, Link, MathBlock, MathInline, OrderedList, UnorderedList,
};

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

    pub fn render(&self, source: &str) -> Html {
        self.render_with_base_path(source, None)
    }

    pub fn render_with_base_path(&self, source: &str, base_dir: Option<&str>) -> Html {
        let processed = self.apply_filters(source);
        let events: Vec<Event> = Parser::new_ext(processed.as_ref(), self.options).collect();
        let html_nodes = render_events(events, base_dir);

        html! { <>{ for html_nodes }</> }
    }

    pub fn render_to_string(&self, source: &str) -> String {
        let processed = self.apply_filters(source);
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

impl MarkdownRenderer {
    fn apply_filters<'a>(&self, source: &'a str) -> Cow<'a, str> {
        let mut processed: Cow<'a, str> = Cow::Borrowed(source);
        for filter in &self.filters {
            let applied = filter.apply(processed.as_ref());
            processed = match applied {
                Cow::Borrowed(_) => processed,
                Cow::Owned(s) => Cow::Owned(s),
            };
        }
        processed
    }
}

fn events_to_html(events: Vec<Event<'_>>) -> Html {
    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());
    Html::from_html_unchecked(AttrValue::from(html_output))
}

fn render_events<'a>(events: Vec<Event<'a>>, base_dir: Option<&str>) -> Vec<Html> {
    let mut nodes: Vec<Html> = Vec::new();
    let mut buffer: Vec<Event<'a>> = Vec::new();
    let mut iter = events.into_iter();
    let mut math_block: Option<String> = None;

    while let Some(event) = iter.next() {
        if let Some(current) = math_block.as_mut() {
            match event {
                Event::Text(text) if text.trim() == "$$" => {
                    nodes.push(html! { <MathBlock formula={current.trim().to_string()} /> });
                    math_block = None;
                }
                Event::Text(text) | Event::Code(text) => {
                    current.push_str(&text);
                }
                Event::SoftBreak | Event::HardBreak => {
                    current.push('\n');
                }
                _ => {}
            }
            continue;
        }

        match event {
            Event::Start(Tag::List(kind)) => {
                if !buffer.is_empty() {
                    nodes.push(events_to_html(buffer));
                    buffer = Vec::new();
                }
                let list_events = collect_list_events(&mut iter);
                nodes.push(render_list_component(kind, list_events, base_dir));
            }
            Event::Start(Tag::BlockQuote) => {
                if !buffer.is_empty() {
                    nodes.push(events_to_html(buffer));
                    buffer = Vec::new();
                }
                let quote_events = collect_blockquote_events(&mut iter);
                let quote_children = render_events(quote_events, base_dir);
                nodes.push(html! { <Blockquote>{ for quote_children }</Blockquote> });
            }
            Event::Start(Tag::Link(_link_type, dest, title)) => {
                if !buffer.is_empty() {
                    nodes.push(events_to_html(buffer));
                    buffer = Vec::new();
                }
                let link_events = collect_link_events(&mut iter);
                let link_children = render_events(link_events, base_dir);
                nodes.push(html! {
                    <Link href={dest.to_string()} title={title.to_string()}>
                        { for link_children }
                    </Link>
                });
            }
            Event::Start(Tag::Image(_link_type, dest, title)) => {
                if !buffer.is_empty() {
                    nodes.push(events_to_html(buffer));
                    buffer = Vec::new();
                }
                let image_events = collect_image_events(&mut iter);
                let alt_text = extract_alt_text(&image_events);
                let rewritten = rewrite_image_src(dest.as_ref(), base_dir);
                nodes.push(html! {
                    <Image
                        src={rewritten}
                        alt={alt_text}
                        title={title.to_string()}
                    />
                });
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                if !buffer.is_empty() {
                    nodes.push(events_to_html(buffer));
                    buffer = Vec::new();
                }
                let code = collect_code_block(&mut iter);
                nodes.push(render_code_block(kind, code));
            }
            Event::Code(text) => buffer.push(Event::Html(CowStr::Boxed(
                render_inline_code_html(text.as_ref()).into_boxed_str(),
            ))),
            Event::Text(text) => {
                if text.trim() == "$$" {
                    if !buffer.is_empty() {
                        nodes.push(events_to_html(buffer));
                        buffer = Vec::new();
                    }
                    math_block = Some(String::new());
                    continue;
                }

                let segments = split_math_segments(&text);
                if segments.len() == 1 {
                    match &segments[0] {
                        MathSegment::Text(_) => buffer.push(Event::Text(text)),
                        MathSegment::InlineMath(formula) => {
                            if !buffer.is_empty() {
                                nodes.push(events_to_html(buffer));
                                buffer = Vec::new();
                            }
                            nodes.push(html! { <MathInline formula={formula.clone()} /> });
                        }
                        MathSegment::BlockMath(formula) => {
                            if !buffer.is_empty() {
                                nodes.push(events_to_html(buffer));
                                buffer = Vec::new();
                            }
                            nodes.push(html! { <MathBlock formula={formula.clone()} /> });
                        }
                    }
                } else {
                    if !buffer.is_empty() {
                        nodes.push(events_to_html(buffer));
                        buffer = Vec::new();
                    }
                    for segment in segments {
                        match segment {
                            MathSegment::Text(t) => nodes.push(html! { t }),
                            MathSegment::InlineMath(formula) => {
                                nodes.push(html! { <MathInline formula={formula} /> })
                            }
                            MathSegment::BlockMath(formula) => {
                                nodes.push(html! { <MathBlock formula={formula} /> })
                            }
                        }
                    }
                }
            }
            other => buffer.push(other),
        }
    }

    if !buffer.is_empty() {
        nodes.push(events_to_html(buffer));
    }

    if let Some(formula) = math_block {
        if !formula.is_empty() {
            nodes.push(html! { <MathBlock formula={formula.trim().to_string()} /> });
        }
    }

    nodes
}

fn collect_list_events<'a, I>(events: &mut I) -> Vec<Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut collected = Vec::new();
    let mut depth = 1;
    while let Some(event) = events.next() {
        match &event {
            Event::Start(Tag::List(_)) => depth += 1,
            Event::End(Tag::List(_)) => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        collected.push(event);
    }
    collected
}

fn collect_item_events<'a, I>(events: &mut I) -> Vec<Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut collected = Vec::new();
    let mut depth = 1;
    while let Some(event) = events.next() {
        match &event {
            Event::Start(Tag::Item) => depth += 1,
            Event::End(Tag::Item) => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        collected.push(event);
    }
    collected
}

fn render_list_component(
    kind: Option<u64>,
    events: Vec<Event<'_>>,
    base_dir: Option<&str>,
) -> Html {
    let mut items: Vec<Html> = Vec::new();
    let mut iter = events.into_iter();
    while let Some(event) = iter.next() {
        if matches!(event, Event::Start(Tag::Item)) {
            let item_events = collect_item_events(&mut iter);
            let item_nodes = render_events(item_events, base_dir);
            items.push(html! { <>{ for item_nodes }</> });
        }
    }

    match kind {
        Some(_) => html! { <OrderedList items={items} /> },
        None => html! { <UnorderedList items={items} /> },
    }
}

fn collect_link_events<'a, I>(events: &mut I) -> Vec<Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut collected = Vec::new();
    let mut depth = 1;
    while let Some(event) = events.next() {
        match &event {
            Event::Start(Tag::Link(..)) => depth += 1,
            Event::End(Tag::Link(..)) => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        collected.push(event);
    }
    collected
}

fn collect_image_events<'a, I>(events: &mut I) -> Vec<Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut collected = Vec::new();
    let mut depth = 1;
    while let Some(event) = events.next() {
        match &event {
            Event::Start(Tag::Image(..)) => depth += 1,
            Event::End(Tag::Image(..)) => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        collected.push(event);
    }
    collected
}

fn extract_alt_text(events: &[Event<'_>]) -> String {
    let mut alt = String::new();
    for event in events {
        match event {
            Event::Text(text) | Event::Code(text) => alt.push_str(text),
            Event::SoftBreak | Event::HardBreak => alt.push(' '),
            _ => {}
        }
    }
    alt.trim().to_string()
}

fn collect_blockquote_events<'a, I>(events: &mut I) -> Vec<Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut collected = Vec::new();
    let mut depth = 1;
    while let Some(event) = events.next() {
        match &event {
            Event::Start(Tag::BlockQuote) => depth += 1,
            Event::End(Tag::BlockQuote) => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        collected.push(event);
    }
    collected
}

fn collect_code_block<'a, I>(events: &mut I) -> String
where
    I: Iterator<Item = Event<'a>>,
{
    let mut code = String::new();
    while let Some(event) = events.next() {
        match event {
            Event::End(Tag::CodeBlock(_)) => break,
            Event::Text(text) | Event::Code(text) => code.push_str(&text),
            Event::SoftBreak | Event::HardBreak => code.push('\n'),
            _ => {}
        }
    }
    code
}

fn render_code_block(kind: CodeBlockKind<'_>, code: String) -> Html {
    let language = match kind {
        CodeBlockKind::Fenced(lang) if !lang.is_empty() => Some(lang.to_string()),
        _ => None,
    };

    if matches!(
        language.as_deref(),
        Some("math" | "latex" | "tex" | "katex")
    ) {
        return html! { <MathBlock formula={code.trim().to_string()} /> };
    }

    html! { <CodeBlock code={code} language={language.map(AttrValue::from)} /> }
}

fn render_inline_code_html(content: &str) -> String {
    format!(
        r#"<code class="rounded bg-white/10 px-1.5 py-0.5 text-sm font-mono text-white/90">{}</code>"#,
        escape_html(content)
    )
}

fn escape_html(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn rewrite_image_src(src: &str, base_dir: Option<&str>) -> String {
    let Some(base_dir) = base_dir else {
        return src.to_string();
    };

    if is_absolute_or_data_url(src) {
        return src.to_string();
    }

    let resolved = resolve_relative_path(base_dir, src);
    format!("/data/{}", resolved)
}

fn is_absolute_or_data_url(src: &str) -> bool {
    src.starts_with('/')
        || src.starts_with("http://")
        || src.starts_with("https://")
        || src.starts_with("data:")
        || src.starts_with("blob:")
        || src.starts_with("mailto:")
}

fn resolve_relative_path(base_dir: &str, src: &str) -> String {
    let mut parts: Vec<&str> = Vec::new();

    if !base_dir.is_empty() {
        parts.extend(base_dir.split('/').filter(|segment| !segment.is_empty()));
    }

    for segment in src.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(segment),
        }
    }

    parts.join("/")
}

enum MathSegment {
    Text(String),
    InlineMath(String),
    BlockMath(String),
}

fn split_math_segments(text: &str) -> Vec<MathSegment> {
    let mut segments = Vec::new();
    let mut cursor = 0;
    let bytes = text.as_bytes();
    let len = bytes.len();

    while cursor < len {
        if bytes[cursor] == b'$' {
            let is_block = cursor + 1 < len && bytes[cursor + 1] == b'$';
            let delim_len = if is_block { 2 } else { 1 };
            let start = cursor + delim_len;
            if start >= len {
                segments.push(MathSegment::Text(text[cursor..].to_string()));
                break;
            }
            let delim_str = if is_block { "$$" } else { "$" };
            if let Some(end_rel) = text[start..].find(delim_str) {
                let end = start + end_rel;
                if cursor > 0 {
                    segments.push(MathSegment::Text(text[..cursor].to_string()));
                }
                let formula = text[start..end].trim().to_string();
                if !formula.is_empty() {
                    if is_block {
                        segments.push(MathSegment::BlockMath(formula));
                    } else {
                        segments.push(MathSegment::InlineMath(formula));
                    }
                }
                cursor = end + delim_len;
                continue;
            } else {
                segments.push(MathSegment::Text(text[cursor..].to_string()));
                break;
            }
        }
        cursor += 1;
    }

    if cursor == len && segments.is_empty() {
        segments.push(MathSegment::Text(text.to_string()));
    } else if cursor < len {
        segments.push(MathSegment::Text(text[cursor..].to_string()));
    }

    segments
}

#[cfg(test)]
mod tests {
    use crate::markdown_renderer::MarkdownRenderer;

    #[test]
    fn strips_double_dash_frontmatter() {
        let src = "--\ntitle: Example\n--\n\n# Heading";
        let html = MarkdownRenderer::new().render_to_string(src);
        assert!(!html.contains("title: Example"));
        assert!(html.contains("<h1>Heading</h1>"));
    }

    #[test]
    fn strips_triple_dash_frontmatter() {
        let src = "---\ntitle: Example\n---\nContent";
        let html = MarkdownRenderer::new().render_to_string(src);
        assert!(!html.contains("title: Example"));
        assert!(html.contains("<p>Content</p>"));
    }

    #[test]
    fn leaves_regular_content() {
        let src = "# No Frontmatter";
        let html = MarkdownRenderer::new().render_to_string(src);
        assert!(html.contains("<h1>No Frontmatter</h1>"));
    }
}
