use std::{cell::Cell, rc::Rc};

use wasm_bindgen_futures::spawn_local;
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
    let highlighted_lines = use_state(|| None::<Vec<String>>);
    let lang_class = props.language.as_deref().map(|l| format!("language-{}", l));

    {
        let highlighted_lines = highlighted_lines.clone();
        use_effect_with((props.code.clone(), props.language.clone()), move |deps| {
            let (code, language) = deps;
            let highlighted_lines = highlighted_lines.clone();
            let code_owned = code.to_string();
            let language_owned = language.clone().map(|value| value.to_string());
            let is_cancelled = Rc::new(Cell::new(false));
            let cancel_flag = is_cancelled.clone();

            highlighted_lines.set(None);

            spawn_local(async move {
                let language_ref = language_owned.as_deref();
                let lines = HighlightService::highlight_lines_html(&code_owned, language_ref).await;

                if !cancel_flag.get() {
                    highlighted_lines.set(Some(lines));
                }
            });

            move || {
                is_cancelled.set(true);
            }
        });
    }

    let render_line = |index: usize, content: Html| {
        html! {
            <span class="grid grid-cols-[auto,1fr] gap-4">
                <span class="w-12 select-none text-right pr-2 text-slate-500 tabular-nums" aria-hidden="true">{ index + 1 }</span>
                <span class="block whitespace-pre">
                    { content }
                </span>
            </span>
        }
    };

    html! {
        <pre class="my-4 overflow-x-auto no-scrollbar rounded-lg bg-black/60 text-sm text-slate-100 border border-[0.5px] border-border bg-card shadow-[0_20px_60px_-25px_rgba(0,0,0,0.85)] backdrop-blur-xl ring-[0.5px] ring-border px-2 py-4">
            <code class={classes!(lang_class, "block", "font-mono", "leading-6", "w-fit")}>
                {
                    if let Some(lines) = &*highlighted_lines {
                        html! {
                            <>
                            {
                                for lines.iter().enumerate().map(|(index, line)| {
                                    render_line(index, Html::from_html_unchecked(AttrValue::from(line.clone())))
                                })
                            }
                            </>
                        }
                    } else {
                        html! {
                            <>
                            {
                                for props.code.as_ref().split_inclusive('\n').enumerate().map(|(index, line)| {
                                    render_line(index, html! { { line } })
                                })
                            }
                            </>
                        }
                    }
                }
            </code>
        </pre>
    }
}
