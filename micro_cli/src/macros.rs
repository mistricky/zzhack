/// Define a command with optional options, subcommands, and a run handler.
///
/// # Example
/// ```
/// use micro_cli::{command, OptionSpec};
///
/// let echo = command! {
///     name: "echo",
///     about: "Print messages",
///     options: [
///         OptionSpec::flag("no-newline", Some('n'), Some("no-newline"), "Skip trailing newline")
///     ],
///     run: |ctx| {
///         let text = ctx.args.join(" ");
///         if ctx.options.flag("no-newline") {
///             print!("{text}");
///         } else {
///             println!("{text}");
///         }
///         Ok(())
///     }
/// };
/// ```
#[macro_export]
macro_rules! command {
    (
        name: $name:expr,
        about: $about:expr,
        $(options: [ $($opt:expr),* $(,)? ],)?
        $(subcommands: [ $($sub:expr),* $(,)? ],)?
        run: |$ctx:ident $(: $ctx_ty:ty)?| $body:block $(,)?
    ) => {{
        let handler: $crate::CommandHandler = std::sync::Arc::new(move |$ctx $(: $ctx_ty)?| $body);
        #[allow(unused_mut)]
        let mut cmd = $crate::Command::new($name, $about, handler);
        $( cmd = cmd.with_options(vec![ $($opt),* ]); )?
        $( cmd = cmd.with_subcommands(vec![ $($sub),* ]); )?
        cmd
    }};
}

/// Define a CLI application with a list of commands.
#[macro_export]
macro_rules! cli {
    (
        name: $name:expr,
        about: $about:expr,
        $(version: $version:expr,)?
        commands: [ $($cmd:expr),* $(,)? ]
    ) => {{
        let mut app = $crate::CliApp::new($name, $about);
        $( app = app.version($version); )?
        app = app.commands(vec![ $($cmd),* ]);
        app
    }};
}
