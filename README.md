# Terminal Emulator

A browser-based terminal emulator built with Rust, Yew framework, and TailwindCSS for modern, responsive styling.

## Features

- **Interactive Terminal Interface**: Full-featured terminal UI with command history
- **Built-in Commands**: 
  - `help` - Display help information for all available commands
  - `echo` - Output text to the terminal
- **Modern Web Technologies**: Built with Rust, Yew, WASM, and TailwindCSS
- **Functional Components**: Modern Yew functional components with hooks
- **Responsive Design**: TailwindCSS for consistent, responsive styling
- **Hot Reload**: Development server with automatic reloading
- **Syntax Highlighting**: Real-time command syntax highlighting with preview

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Node.js](https://nodejs.org/) (for npx to run TailwindCSS)
- [Trunk](https://trunkrs.dev/) - WASM web application bundler

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd terminal-emulator
   ```

2. Install Trunk (if not already installed):
   ```bash
   cargo install --locked trunk
   ```

Note: TailwindCSS will be automatically downloaded and run via `npx` during the build process - no local installation needed!

## Development

### Option 1: Use the development script (recommended)
```bash
./dev.sh
```

### Option 2: Manual steps
1. Generate TailwindCSS styles:
   ```bash
   ./generate-css.sh
   ```

2. Start the development server:
   ```bash
   trunk serve --open
   ```

The application will be available at `http://127.0.0.1:8080`. 

**Note**: When you modify Tailwind classes in your Rust code, run `./generate-css.sh` to regenerate the CSS styles.

## Building for Production

Build the application for production deployment:

```bash
./build.sh
# Or manually:
trunk build --release
```

The built files will be available in the `dist` directory.

## Usage

Once the application is running:

1. Click in the terminal input field or it should be automatically focused
2. Type commands and press Enter to execute them
3. Available commands:
   - `help` - Show all available commands and their usage
   - `echo <text>` - Output the specified text

## Project Structure

```
src/
├── lib.rs              # Library root and module exports
├── main.rs             # WASM entry point
├── app.rs              # Main App component (functional)
├── components/
│   ├── mod.rs          # Components module
│   └── terminal.rs     # Terminal UI components (functional)
└── commands/
    └── mod.rs          # Command system and built-in commands

Trunk.toml              # Trunk configuration
Cargo.toml              # Rust project configuration
index.html              # HTML template
input.css               # TailwindCSS input file
tailwind.config.js      # TailwindCSS configuration
build.sh                # Production build script
dev.sh                  # Development script
generate-css.sh         # TailwindCSS generation script
```

## Architecture

- **Frontend**: Yew functional components with hooks for reactive UI
- **Styling**: TailwindCSS for utility-first, responsive design
- **Command System**: Extensible command architecture with trait-based design
- **Build System**: Trunk for WASM bundling with TailwindCSS integration
- **State Management**: Yew hooks (use_state, use_effect) for component state

## Routing

Routes are defined in `App.toml` and executed via the terminal command runner:

- Static match: `/` runs its command exactly.
- Named segment: `/posts/{name}/foo` binds `{name}` to the segment between `posts` and `foo` (e.g., `/posts/bar/foo` sets `name = "bar"`).
- Wildcard tail: `/posts/{*file_path}` captures everything after `/posts/` (e.g., `/posts/a/b/c` sets `file_path = "/a/b/c"`, and visiting `/posts/` sets `file_path = "/"`).
- Fallback: `*` matches anything not caught earlier.

Captured values can be interpolated into commands using `{name}` or `{*name}`.

## Adding New Commands

To add a new command:

1. Create a struct that implements the `Command` trait in `src/commands/mod.rs`
2. Register the command in `CommandExecutor::new()`
3. The command will automatically be available in the terminal

Example:
```rust
pub struct MyCommand;
impl Command for MyCommand {
    fn execute(&self, args: &[String]) -> CommandResult {
        CommandResult::Success("Hello from my command!".to_string())
    }
    
    fn description(&self) -> &'static str {
        "My custom command"
    }
    
    fn usage(&self) -> &'static str {
        "mycommand"
    }
}
```

## License

MIT License
