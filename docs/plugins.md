# Plugins

`soroban-forge` follows git's model for extension: any executable named
`soroban-forge-<name>` on `PATH` becomes available as `soroban-forge <name>`.
There's no plugin registry or install command — building a plugin is writing
an ordinary Rust (or any-language) binary and putting it on `PATH`.

```sh
soroban-forge hello Ada    # -> execs `soroban-forge-hello Ada` if found on PATH
soroban-forge --list       # shows built-in subcommands and any discovered
                            # `soroban-forge-*` binaries under "External:"
```

See [Writing a soroban-forge plugin](plugin-tutorial.md) for a full,
start-to-finish walkthrough — crate layout, the `ForgePlugin` trait, error
handling, exit codes, and a compiling example crate.
