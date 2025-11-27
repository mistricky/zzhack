use yew::prelude::*;

use crate::highlight_service::HighlightService;

#[derive(Properties, PartialEq)]
pub struct CodeBlockProps {
    pub code: AttrValue,
    #[prop_or_default]
    pub language: Option<AttrValue>,
}

#[function_component(CodeBlock)]
pub fn code_block(props: &CodeBlockProps) -> Html {
    let lang = props.language.as_deref();
    let highlighted = HighlightService::highlight_html(&props.code, lang);
    let lang_class = lang.map(|l| format!("language-{}", l)).unwrap_or_default();
    let wrapped = format!(
        "<pre class=\"my-4 overflow-x-auto rounded-lg bg-black/60 p-4 text-sm text-slate-100\"><code class=\"{lang_class}\">{highlighted}</code></pre>"
    );

    Html::from_html_unchecked(AttrValue::from(wrapped))
}
