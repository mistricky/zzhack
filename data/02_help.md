You just saw Hello world printed on the screen.
Feels familiar, doesn’t it?

It may look like a regular shell command, but this command is actually running inside zzhack.
Every command you type is mapped to an implementation under `app/src/commands`.

If a command implements the ExecutableCommand trait, zzhack knows how to execute it.
Here’s a simplified version of how `echo` is implemented:

[app/src/commands/echo.rs](https://github.com/mistricky/zzhack/blob/main/app/src/commands/echo.rs)
```rust
#[derive(Parser, Debug, Default)]
#[command(name = "echo", about = "Print the given text to the console")]
pub struct EchoCommand {
    #[arg(positional, help = "Text to echo")]
    message: Vec<String>,
}

impl ExecutableCommand<CommandContext> for EchoCommand {
    fn run(&self, args: &[String], ctx: &CommandContext) -> Result<(), String> {
        let Some(cli) = parse_cli::<EchoCommand>(args, ctx, self.command_name()) else {
            return Ok(());
        };
        let msg = cli.message.join(" ");

        console::log_1(&msg.clone().into());
        ctx.terminal.push_text(msg);
        Ok(())
    }
}
```


Notice that zzhack isn’t trying to be a full shell, the parser is intentionally minimal—and that’s a feature, not a limitation.but for a terminal-style personal website, it’s more than enough.

Curious what else you can run?

Try run `help` to see a list of available commands.
