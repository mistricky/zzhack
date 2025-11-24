use pulldown_cmark::{html, Options, Parser};

pub fn render_markdown_to_html(source: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(source, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
