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
    let highlighted = HighlightService::highlight_lines_html(&props.code, lang);
    let lang_class = lang.map(|l| format!("language-{}", l));

    html! {
        <pre class="my-4 overflow-x-auto rounded-lg bg-black/60 text-sm text-slate-100">
            <code class={classes!(lang_class, "block", "font-mono", "leading-6")}>
                {
                    for highlighted.iter().enumerate().map(|(index, line)| html! {
                        <span class="grid grid-cols-[auto,1fr] gap-4">
                            <span class="w-12 select-none text-right pr-2 text-slate-500 tabular-nums" aria-hidden="true">{ index + 1 }</span>
                            <span class="block whitespace-pre">
                                { Html::from_html_unchecked(AttrValue::from(line.clone())) }
                            </span>
                        </span>
                    })
                }
            </code>
        </pre>
    }
}
