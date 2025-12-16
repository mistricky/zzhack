Before wrapping up, let’s take a look at `App.toml`.

`App.toml` contains a wide range of configuration options.
For example, it stores author-related information such as name and email.

When you run the `whoami` command, zzhack prints author.name.
When you run the `email` command, it prints author.email.
There are many more configuration fields waiting to be explored.

The most important configuration, however, is `app.routes`.

`app.routes` allows you to define routes as commands.
When zzhack navigates to a specific path, it interprets that path using `app.routes` and then executes the command configured for it.

```toml
routes = [
  { path = "/", command = 'clear && render -r 01_greeter.md'},
  { path = "*", command = 'clear && echo "Not found!"' }
]
```

In other words, routing in zzhack is just command execution.

This means a route can run any command.

Once you combine this with render or any other command, you can start crafting your own personal website or blog entirely through commands.

That’s it.

Have fun hacking, and enjoy building with zzhack! ♥️
