use std::{cell::Cell, rc::Rc};

use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TypewriterProps {
    pub content: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(45)]
    pub delay_ms: u32,
}

#[derive(Clone)]
enum HtmlToken {
    Tag(String),
    Text(String),
}

fn tokenize_html(input: &str) -> Vec<HtmlToken> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut chars = input.chars();

    while let Some(ch) = chars.next() {
        if ch == '<' {
            if !buffer.is_empty() {
                tokens.push(HtmlToken::Text(std::mem::take(&mut buffer)));
            }
            let mut tag = String::from('<');
            while let Some(next) = chars.next() {
                tag.push(next);
                if next == '>' {
                    break;
                }
            }
            tokens.push(HtmlToken::Tag(tag));
        } else {
            buffer.push(ch);
        }
    }

    if !buffer.is_empty() {
        tokens.push(HtmlToken::Text(buffer));
    }

    tokens
}

#[function_component(Typewriter)]
pub fn typewriter(props: &TypewriterProps) -> Html {
    let rendered_text = use_state(String::new);
    let template_ref = use_node_ref();
    use_effect_with((props.content.clone(), props.delay_ms), {
        let rendered_text = rendered_text.clone();
        let template_ref = template_ref.clone();
        move |(content, delay_ms)| {
            let _ = content;
            rendered_text.set(String::new());

            let text = template_ref
                .cast::<Element>()
                .map(|element| element.inner_html())
                .unwrap_or_default();
            let delay = *delay_ms;
            let mut cleanup_flag = None;

            if !text.is_empty() {
                let tokens = tokenize_html(&text);
                let total_tokens = tokens.len();
                let is_running = Rc::new(Cell::new(true));
                cleanup_flag = Some(Rc::clone(&is_running));
                let handle = rendered_text.clone();

                spawn_local({
                    let run_flag = Rc::clone(&is_running);
                    let handle = handle;
                    let total_tokens = total_tokens;
                    let tokens = tokens;
                    async move {
                        let mut buffer = String::new();
                        for (token_index, token) in tokens.into_iter().enumerate() {
                            if !run_flag.get() {
                                break;
                            }

                            match token {
                                HtmlToken::Tag(tag) => {
                                    buffer.push_str(&tag);
                                    handle.set(buffer.clone());
                                }
                                HtmlToken::Text(text_segment) => {
                                    let characters: Vec<char> = text_segment.chars().collect();
                                    let total_chars = characters.len();
                                    for (char_index, ch) in characters.into_iter().enumerate() {
                                        if !run_flag.get() {
                                            break;
                                        }

                                        buffer.push(ch);
                                        handle.set(buffer.clone());

                                        let is_last_char = char_index + 1 == total_chars;
                                        let is_last_token = token_index + 1 == total_tokens;
                                        if !(is_last_char && is_last_token) {
                                            TimeoutFuture::new(delay).await;
                                        }
                                    }
                                }
                            }
                        }
                    }
                });
            }

            move || {
                if let Some(flag) = cleanup_flag {
                    flag.set(false);
                }
            }
        }
    });

    let typed_html = Html::from_html_unchecked(AttrValue::from((*rendered_text).clone()));

    html! {
        <span class={props.class.clone()}>
            { typed_html }
            <span ref={template_ref} style="display: none;" aria-hidden="true">
                { props.content.clone() }
            </span>
        </span>
    }
}
