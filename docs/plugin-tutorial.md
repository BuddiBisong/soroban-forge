# Writing a soroban-forge plugin

A start-to-finish walkthrough for building a third-party `soroban-forge`
subcommand — no fork or PR into this repository required. The full, compiling
example this tutorial builds lives at
[`docs/examples/hello-plugin`](examples/hello-plugin); every code block below
is copied verbatim from it.

If you'd rather contribute a first-party module (one of the crates under
`crates/*`, wired directly into `src/main.rs`), see
["How the plugin interface works"](../CONTRIBUTING.md#how-the-plugin-interface-works)
in CONTRIBUTING.md instead — the `ForgePlugin` trait is the same either way;
what differs is only how your command gets discovered.

## How third-party discovery works

`soroban-forge` follows the same convention as `git`: when you run
`soroban-forge <name> ...` and `<name>` isn't one of its built-in
subcommands, it looks for a binary called `soroban-forge-<name>` on `PATH`
and executes it, forwarding the remaining arguments and inheriting stdio.
Your plugin's exit code becomes `soroban-forge`'s exit code. This is
implemented in
[`crates/core/src/cli.rs`](../crates/core/src/cli.rs)'s `try_run_external`
and `find_external_subcommands`.

Two consequences worth knowing up front:

- Your binary receives argv **without** the subcommand name. `soroban-forge
  hello ada --loud` execs `soroban-forge-hello ada --loud` — your plugin's
  own `clap::Command` is the *root* command, not a subcommand of anything.
- Four global flags forward as environment variables rather than argv:
  `--verbose`/`--quiet`/`--json`/`--yes` become `SOROBAN_FORGE_VERBOSE`,
  `SOROBAN_FORGE_QUIET`, `SOROBAN_FORGE_JSON`, `SOROBAN_FORGE_YES` (each set
  to `1` when the flag was passed). A well-behaved plugin reads these the
  same way the built-in subcommands honor the flags directly.

## 1. Crate layout

A plugin is an ordinary Rust binary crate. Nothing about its layout is
special beyond the binary's name matching `soroban-forge-<name>`:

```toml
# Cargo.toml
[package]
name = "soroban-forge-hello"
version = "0.1.0"
edition = "2021"

[dependencies]
soroban-forge-core = "0.1"   # published crate; a real plugin's repo has no
                              # reason to path-depend on soroban-forge's tree
clap = "4.5"

[[bin]]
name = "soroban-forge-hello"
path = "src/main.rs"
```

`soroban-forge-core` is the only thing you need from this project: the
`ForgePlugin` trait, `ForgeContext` (config/cwd/output-mode access), and
`ForgeError`/`Result` (see the next two sections). It's the same core every
first-party module builds on — nothing about it is off-limits to an
external crate.

## 2. Implement `ForgePlugin`

```rust
pub trait ForgePlugin {
    fn name(&self) -> &'static str;      // subcommand name
    fn command(&self) -> clap::Command;  // clap definition
    fn run(&self, matches: &ArgMatches, ctx: &ForgeContext) -> Result<()>;
}
```

`name()` is the subcommand word (`"hello"` — the part after
`soroban-forge-`, not the binary name itself). `command()` builds the
`clap::Command` describing your flags and arguments — exactly what you'd
write for any standalone clap CLI:

```rust
fn command(&self) -> Command {
    Command::new("hello")
        .about("Print a greeting, optionally naming the project from forge.toml")
        .arg(Arg::new("name").help(
            "Who to greet [default: [project] name from forge.toml, or \"world\"]",
        ))
        .arg(
            Arg::new("loud")
                .long("loud")
                .action(clap::ArgAction::SetTrue)
                .help("Shout the greeting"),
        )
}
```

`ctx: &ForgeContext` is how `run()` sees the invocation environment: `ctx.cwd`
(the working directory), `ctx.config` (the parsed `forge.toml`, if one was
found — see [the forge.toml reference](configuration.md#forgetoml-reference)
for every key any plugin can read), `ctx.quiet`/`ctx.json` (so your output
respects the same global flags every other subcommand does), and
`ctx.offline` (skip network calls when set). This example reads
`ctx.config`'s `[project] name` as a fallback when no `NAME` argument was
given:

```rust
fn run(&self, matches: &ArgMatches, ctx: &ForgeContext) -> Result<()> {
    let name = match matches.get_one::<String>("name") {
        Some(name) if name.trim().is_empty() => {
            return Err(ForgeError::InvalidArgument(
                "NAME must not be empty".to_string(),
            ));
        }
        Some(name) => name.clone(),
        None => ctx
            .config
            .as_ref()
            .and_then(|c| c.project.name.clone())
            .unwrap_or_else(|| "world".to_string()),
    };

    let greeting = format!("Hello, {name}!");
    if matches.get_flag("loud") {
        println!("{}", greeting.to_uppercase());
    } else {
        println!("{greeting}");
    }
    Ok(())
}
```

## 3. Return `ForgeError`, never exit directly

Notice `run()` above returns `Err(ForgeError::InvalidArgument(...))` for a
bad argument instead of printing a message and calling
`std::process::exit(1)` itself. This is the one hard rule every plugin —
first-party or third-party — is expected to follow (see
[CONTRIBUTING.md](../CONTRIBUTING.md#ground-rules)):

> Plugins report failures by returning `Err(ForgeError::...)`, never by
> calling `std::process::exit` themselves — the binary derives the process
> exit code from the error variant, and a plugin exiting directly would
> bypass that.

Two things make this worth the discipline:

- **Testability.** A `run()` that returns `Result<()>` can be unit tested by
  asserting on the returned `Err` variant. A `run()` that calls
  `std::process::exit` can't be tested in-process at all — the test runner's
  own process would exit.
- **Consistent exit codes.** [`ForgeError`](../crates/core/src/error.rs) maps
  each variant to one of soroban-forge's four stable exit codes via
  `ForgeError::exit_code()` (documented in full in
  [docs/exit-codes.md](exit-codes.md)):

  | code | meaning        | which `ForgeError` variants |
  |------|----------------|------------------------------|
  | `1`  | user error     | `InvalidArgument`, `Config`, `AlreadyExists`, `VerificationFailed`, `SizeBudgetExceeded` |
  | `2`  | tool missing   | `ToolMissing`, `ToolUnsupported`, `Doctor` |
  | `3`  | internal error | `Io`, `Template`, `Other` |

  Pick the variant that matches what actually went wrong — `InvalidArgument`
  for bad input, `ToolMissing`/`ToolUnsupported` if your plugin shells out to
  something that isn't installed or is too old, `Io` for filesystem
  failures — and every consumer of your plugin (a human, a CI script
  branching on `$?`) gets the same stable contract every built-in subcommand
  already gives them.

## 4. Wire up `main()`

A first-party module's `run()` is dispatched by `soroban-forge`'s own
`src/main.rs`, which already does the argument parsing, `ForgeContext`
construction, and exit-code mapping once for every plugin. A standalone
plugin binary has no such wrapper — it *is* the whole process — so its
`main()` does that same job for itself, for itself alone:

```rust
fn main() {
    let plugin = HelloPlugin;
    let cmd = plugin.command().name("soroban-forge-hello");
    let matches = cmd.get_matches();

    let verbose = std::env::var_os("SOROBAN_FORGE_VERBOSE").is_some() as u8;
    let quiet = std::env::var_os("SOROBAN_FORGE_QUIET").is_some();
    let json = std::env::var_os("SOROBAN_FORGE_JSON").is_some();
    let yes = std::env::var_os("SOROBAN_FORGE_YES").is_some();

    let cwd = std::env::current_dir().expect("current directory");
    let ctx = match ForgeContext::with_output(cwd, verbose, quiet, json, yes) {
        Ok(ctx) => ctx,
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(err.exit_code().into());
        }
    };

    if let Err(err) = plugin.run(&matches, &ctx) {
        eprintln!("error: {err}");
        std::process::exit(err.exit_code().into());
    }
}
```

`ForgeContext::with_output` discovers `forge.toml` the same way every
built-in subcommand does (walking up from `cwd`), so `ctx.config` behaves
identically whether your plugin is invoked as `soroban-forge hello` or
directly as `soroban-forge-hello`.

## 5. Test it

Because `run()` is a plain function returning `Result<()>`, it's testable
without spawning a process:

```rust
#[test]
fn empty_name_is_rejected() {
    let cmd = HelloPlugin.command();
    let matches = cmd.try_get_matches_from(["hello", ""]).unwrap();
    let ctx = ForgeContext::new(std::env::temp_dir(), 0).unwrap();
    assert!(matches!(
        HelloPlugin.run(&matches, &ctx),
        Err(ForgeError::InvalidArgument(_))
    ));
}
```

## 6. Build, install, and try it

```sh
cargo build --release
# put the binary on PATH, e.g.:
cp target/release/soroban-forge-hello ~/.cargo/bin/

soroban-forge hello           # -> Hello, world!
soroban-forge hello Ada --loud   # -> HELLO, ADA!
soroban-forge hello ""        # -> error: invalid argument: NAME must not be empty (exit 1)
soroban-forge --list          # your plugin appears under "External:"
```

`soroban-forge --list` reads the same `PATH` scan `try_run_external` uses,
so it's a quick way to confirm your binary is discoverable before ever
invoking it.
