You just saw Hello world printed on the screen.
Feels familiar, doesn’t it?

It may look like a regular shell command, but this command is actually running inside zzhack.
Every command you type is mapped to an implementation under app/src/commands.

If a command implements the ExecutableCommand trait, zzhack knows how to execute it.
Here’s a simplified version of how echo is implemented:

xxxxxx


zzhack isn’t trying to be a full shell.
The parser is intentionally minimal—and that’s a feature, not a limitation.
For a terminal-style personal website, it’s more than enough.

Curious what else you can run?

Try run `help` to see a list of available commands.
