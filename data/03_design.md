Pretty interesting, right?

To really understand zzhack, there’s one core idea you need to get first:
zzhack is command-driven by design.

Everything in zzhack happens through commands.
Every action—viewing content, navigating pages, rendering text—is expressed as a command, or a composition of commands.

For example, the cat command isn’t doing anything magical.
Conceptually, basically it’s just:

```shell
fetch && echo
```

Once a command implements the ExecutableCommand trait, it becomes a first-class citizen in zzhack.
You can create your own commands, compose them with existing ones, and zzhack will even generate the corresponding help output for you automatically.

At this point, you might be wondering:

What does all of this have to do with a “personal website” or a blog?

To answer that, try running the `render` command.

The `render` command takes a Markdown file and renders it directly into the terminal.
In fact, the article you’re reading right now is rendered exactly this way.

Behind the scenes, this page is built by hacking a few existing commands and then invoking render to display the result.
Those customizations live in data/.shrc.

Yes—just like a real shell, zzhack has its own rc file.

If you’re curious, try taking a look yourself: `cat .shrc`
