//! `soroban-forge-hello` — a minimal third-party soroban-forge plugin.
//!
//! Once this binary is on `PATH`, `soroban-forge hello <NAME>` invokes it
//! (soroban-forge discovers any `soroban-forge-<name>` binary on `PATH` and
//! execs it with the remaining arguments — see `docs/plugin-tutorial.md`).
//! It also runs standalone: `soroban-forge-hello <NAME>`.

use clap::{Arg, ArgMatches, Command};
use soroban_forge_core::{ForgeContext, ForgeError, ForgePlugin, Result};

pub struct HelloPlugin;

impl ForgePlugin for HelloPlugin {
    fn name(&self) -> &'static str {
        "hello"
    }

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

    fn run(&self, matches: &ArgMatches, ctx: &ForgeContext) -> Result<()> {
        let name = match matches.get_one::<String>("name") {
            Some(name) if name.trim().is_empty() => {
                // Return a ForgeError instead of exiting directly (see
                // docs/plugin-tutorial.md) — the caller decides how to
                // report it and which process exit code it maps to.
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
}

fn main() {
    // A standalone plugin binary is its own tiny CLI, not a subcommand
    // nested under `soroban-forge`'s own `Command` tree: soroban-forge's
    // dispatcher already strips `hello` off argv before exec'ing this
    // binary, so `HelloPlugin::command()` is parsed here as the *root*
    // command.
    let plugin = HelloPlugin;
    let cmd = plugin.command().name("soroban-forge-hello");
    let matches = cmd.get_matches();

    // soroban-forge forwards global flags (--verbose/--quiet/--json/--yes)
    // to external subcommands as SOROBAN_FORGE_* environment variables, so
    // a well-behaved plugin honors them the same way the built-in ones do.
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
        // The exit code comes from `ForgeError::exit_code()`, not a
        // hardcoded number — this is what keeps exit codes consistent with
        // soroban-forge's own subcommands (docs/exit-codes.md).
        eprintln!("error: {err}");
        std::process::exit(err.exit_code().into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn defaults_to_world_without_a_name_or_config() {
        let cmd = HelloPlugin.command();
        let matches = cmd.try_get_matches_from(["hello"]).unwrap();
        let ctx = ForgeContext::new(std::env::temp_dir(), 0).unwrap();
        assert!(HelloPlugin.run(&matches, &ctx).is_ok());
    }
}
