use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CodeBlockProps {
    pub code: AttrValue,
    #[prop_or_default]
    pub language: Option<AttrValue>,
}

#[function_component(CodeBlock)]
pub fn code_block(props: &CodeBlockProps) -> Html {
    let lang_class = props
        .language
        .as_ref()
        .map(|lang| format!("language-{}", lang))
        .unwrap_or_default();

    html! {
        <pre class="my-4 overflow-x-auto rounded-lg bg-black/60 p-4 text-sm text-slate-100">
            <code class={lang_class}>{ props.code.clone() }</code>
        </pre>
    }
}
