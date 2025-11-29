use crate::config_service::{ConfigService, RouteSection};
use crate::terminal::Terminal;
use std::collections::HashMap;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, Event, History};

pub struct RouterHandle {
    _popstate: Closure<dyn FnMut(Event)>,
}

pub fn start_router(terminal: Terminal) -> Option<RouterHandle> {
    let win = window()?;
    let handler_terminal = terminal.clone();
    let popstate = Closure::wrap(Box::new(move |_event: Event| {
        if let Some(path) = current_path() {
            run_route(&path, handler_terminal.clone());
        }
    }) as Box<dyn FnMut(_)>);

    win.add_event_listener_with_callback("popstate", popstate.as_ref().unchecked_ref())
        .ok()?;

    if let Some(path) = current_path() {
        run_route(&path, terminal);
    }

    Some(RouterHandle {
        _popstate: popstate,
    })
}

pub fn run_route(path: &str, terminal: Terminal) {
    let normalized = normalize_path(path);
    let routes = &ConfigService::get().app.routes;
    if let Some(command) = resolve_command(&normalized, routes) {
        spawn_local(async move {
            terminal.execute_command(&command).await;
        });
    }
}

fn resolve_command(path: &str, routes: &[RouteSection]) -> Option<String> {
    for route in routes {
        if let Some(command) = match_route(path, route) {
            return Some(command);
        }
    }
    None
}

fn match_route(path: &str, route: &RouteSection) -> Option<String> {
    if route.path == "*" {
        return Some(route.command.clone());
    }

    let pattern = normalize_path(&route.path);
    let path_segments: Vec<&str> = segments(path);
    let pattern_segments: Vec<&str> = segments(&pattern);

    let mut params = HashMap::new();
    let mut path_idx = 0usize;

    for part in pattern_segments.iter() {
        match placeholder_kind(part) {
            Some(Placeholder::Wildcard(name)) => {
                let remaining = &path_segments[path_idx..];
                let joined = if remaining.is_empty() {
                    "/".to_string()
                } else {
                    format!("/{}", remaining.join("/"))
                };
                params.insert(name.to_string(), joined);
                path_idx = path_segments.len();
                break;
            }
            Some(Placeholder::Single(name)) => {
                let value = path_segments.get(path_idx)?;
                params.insert(name.to_string(), (*value).to_string());
                path_idx += 1;
            }
            None => {
                let value = path_segments.get(path_idx)?;
                if part != value {
                    return None;
                }
                path_idx += 1;
            }
        }
    }

    if path_idx != path_segments.len() {
        return None;
    }

    Some(apply_params(&route.command, &params))
}

fn apply_params(template: &str, params: &HashMap<String, String>) -> String {
    let mut resolved = template.to_string();
    for (key, value) in params {
        resolved = resolved.replace(&format!("{{{key}}}"), value);
        resolved = resolved.replace(&format!("{{*{key}}}"), value);
    }
    resolved
}

enum Placeholder<'a> {
    Single(&'a str),
    Wildcard(&'a str),
}

fn placeholder_kind(segment: &str) -> Option<Placeholder<'_>> {
    if let Some(name) = segment
        .strip_prefix("{*")
        .and_then(|s| s.strip_suffix('}'))
        .filter(|s| !s.is_empty())
    {
        return Some(Placeholder::Wildcard(name));
    }

    segment
        .strip_prefix('{')
        .and_then(|s| s.strip_suffix('}'))
        .filter(|s| !s.is_empty())
        .map(Placeholder::Single)
}

fn normalize_path(path: &str) -> String {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return "/".to_string();
    }
    if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{}", trimmed)
    }
}

fn segments(path: &str) -> Vec<&str> {
    path.trim_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .collect()
}

fn current_path() -> Option<String> {
    window()
        .and_then(|w| w.location().pathname().ok())
        .map(|p| normalize_path(&p))
}

#[allow(dead_code)]
fn browser_history() -> Option<History> {
    window().and_then(|w| w.history().ok())
}
