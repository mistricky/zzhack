# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs`: application entry; configures tracing and mounts `App`.
- `src/app.rs`: primary Yew component; break new UI into additional modules under `src/`.
- `src/styles/global.css`: Tailwind entry point; utilities emitted via JIT per `tailwind.config.js`.
- `index.html`: Trunk entry HTML; update `<title>`/meta as the app evolves.
- Build outputs: `dist/` (Trunk) and `target/` (Cargo) are generated and gitignored.
- Configuration roots: `Cargo.toml`, `Trunk.toml`, `tailwind.config.js`.

## Build, Test, and Development Commands
- `rustup target add wasm32-unknown-unknown`: one-time setup for the WASM target.
- `trunk serve`: dev server with live reload at `http://127.0.0.1:8080`.
- `trunk watch`: rebuild on change without hosting (useful when embedding elsewhere).
- `trunk build --release`: optimized production build written to `dist/`.
- `cargo fmt`: format Rust sources; run before committing.
- `cargo clippy --target wasm32-unknown-unknown -- -D warnings`: lint with warnings as errors.

## Coding Style & Naming Conventions
- Rust 2021; 4-space indentation; `snake_case` for functions/vars, `PascalCase` for types/components, `SCREAMING_SNAKE_CASE` for constants.
- Keep Yew components small and typed; prefer `#[function_component]` for UI pieces and keep props explicit.
- Tailwind utilities stay in markup; add shared styles in `src/styles/global.css`.
- Log via `tracing`; set meaningful targets and keep noisy logs out of hot paths.

## Testing Guidelines
- No automated tests yet; add `#[cfg(test)]` modules alongside logic or integration tests under `tests/`.
- Use `cargo test` for non-WASM logic; for browser-facing behavior introduce `wasm-bindgen-test` when needed.
- Name tests after behavior (e.g., `renders_header`) and keep assertions focused.

## Commit & Pull Request Guidelines
- No existing history; prefer clear, imperative commits (e.g., `feat: add hero component`, `chore: run cargo fmt`).
- Before PR: run `cargo fmt`, `cargo clippy --target wasm32-unknown-unknown`, and a release build (`trunk build --release`).
- PR description should summarize changes, list manual checks, and include screenshots/recordings for UI updates.
- Link related issues and call out follow-up work or known limitations.

## Agent Operating Principles
- Think first: design abstractions and file layout before coding.
- Post-generation checklist: 1) fix warnings (`cargo check`, `rust-analyzer diagnostics <file>`); 2) self code review for clarity and correctness.
- Code hygiene: keep files/modules small and single-purpose, minimize side effects, and add brief comments only for non-obvious logic.
- Tooling discipline: prefer fast searches (`rg`), use `apply_patch` for single-file edits, and avoid destructive git commands unless explicitly requested.
