use once_cell::sync::Lazy;
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, Theme, ThemeSet},
    html::{append_highlighted_html_for_styled_line, IncludeBackground},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

pub struct HighlightService;

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| SyntaxSet::load_defaults_newlines());
static THEME: Lazy<Theme> = Lazy::new(|| {
    ThemeSet::load_defaults()
        .themes
        .get("base16-ocean.dark")
        .cloned()
        .unwrap_or_else(|| ThemeSet::load_defaults().themes["base16-eighties.dark"].clone())
});

impl HighlightService {
    pub fn highlight_html(code: &str, language: Option<&str>) -> String {
        Self::highlight_lines_html(code, language).join("\n")
    }

    pub fn highlight_lines_html(code: &str, language: Option<&str>) -> Vec<String> {
        let syntax = language
            .and_then(|lang| SYNTAX_SET.find_syntax_by_token(lang))
            .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());

        let mut highlighter = HighlightLines::new(syntax, &THEME);
        let mut output = Vec::new();

        for line in LinesWithEndings::from(code) {
            let mut highlighted_line = String::new();
            if let Ok(ranges) = highlighter.highlight_line(line, &SYNTAX_SET) {
                let _ = append_highlighted_html_for_styled_line(
                    &ranges,
                    IncludeBackground::No,
                    &mut highlighted_line,
                );
            } else {
                let _ = append_highlighted_html_for_styled_line(
                    &[(Style::default(), line)],
                    IncludeBackground::No,
                    &mut highlighted_line,
                );
            }

            let highlighted_line = highlighted_line
                .strip_suffix('\n')
                .unwrap_or(&highlighted_line)
                .to_owned();
            output.push(highlighted_line);
        }

        output
    }
}
