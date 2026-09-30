use soroban_forge_core::ForgePlugin;

fn main() {
    let plugins: Vec<Box<dyn ForgePlugin>> = vec![
        Box::new(soroban_forge_core::ConfigPlugin),
        Box::new(soroban_forge_init::InitPlugin),
        Box::new(soroban_forge_scaffold::ScaffoldPlugin),
        Box::new(soroban_forge_testgen::TestgenPlugin),
        Box::new(soroban_forge_ci_presets::CiPresetsPlugin),
        Box::new(soroban_forge_doctor::DoctorPlugin),
        Box::new(soroban_forge_bindings_ts::BindingsTsPlugin),
        Box::new(soroban_forge_bindings_py::BindingsPyPlugin),
        Box::new(soroban_forge_templates::TemplatesPlugin),
        Box::new(soroban_forge_spec::SpecPlugin),
        Box::new(soroban_forge_verify::VerifyPlugin),
        Box::new(soroban_forge_identity::IdentityPlugin),
        Box::new(soroban_forge_deploy::DeployPlugin),
        Box::new(soroban_forge_invoke::InvokePlugin),
        Box::new(soroban_forge_network::NetworkPlugin),
        Box::new(soroban_forge_optimize::OptimizePlugin),
    ];

    // Intercept `completions <shell>` before full dispatch so we have access
    // to the complete clap `Command` tree (needed by clap_complete::generate).
    let raw_args: Vec<String> = std::env::args().collect();
    if raw_args.get(1).map(String::as_str) == Some("completions") {
        use clap_complete::{generate, shells};

        let shell_arg = raw_args.get(2).map(String::as_str).unwrap_or("");
        let mut cmd = soroban_forge_core::cli::build_command(&plugins);
        let mut stdout = std::io::stdout();
        match shell_arg {
            "bash" => generate(shells::Bash, &mut cmd, "soroban-forge", &mut stdout),
            "zsh" => generate(shells::Zsh, &mut cmd, "soroban-forge", &mut stdout),
            "fish" => generate(shells::Fish, &mut cmd, "soroban-forge", &mut stdout),
            "powershell" => generate(shells::PowerShell, &mut cmd, "soroban-forge", &mut stdout),
            other => {
                eprintln!(
                    "error: unknown shell `{other}` — supported: bash, zsh, fish, powershell"
                );
                std::process::exit(1);
            }
        }
        return;
    }

    // Sniff the --offline flag and the forge.toml update_check setting before
    // full argument parsing so we can spawn the background check early.  We
    // read the raw args directly here to avoid duplicating clap parsing.
    let offline = raw_args.iter().any(|a| a == "--offline");
    let update_check_enabled = update_check_enabled_from_env_and_config();

    // Spawn the background version-check thread. It returns immediately.
    let update_handle = soroban_forge_update_check::spawn(offline, update_check_enabled);

    if let Err(err) = soroban_forge_core::run(plugins) {
        eprintln!("error: {err}"); // logged
        // Print any pending update hint before exiting with an error.
        soroban_forge_update_check::wait_and_print(update_handle);
        std::process::exit(err.exit_code().into());
    }

    // Print the update hint (if any) after the command has finished.
    soroban_forge_update_check::wait_and_print(update_handle);
}

/// Check whether the update check should run based on environment variables
/// and the forge.toml `[defaults] update_check` key.
///
/// Environment variable `SOROBAN_FORGE_NO_UPDATE_CHECK=1` is the per-session
/// opt-out; the forge.toml key is the persistent opt-out.  Both are honoured
/// in `soroban_forge_update_check::spawn` as well, but we mirror the config
/// key here so that the binary never needs to parse TOML just to decide
/// whether to even spawn the thread.
fn update_check_enabled_from_env_and_config() -> bool {
    // Per-session env opt-out.
    if std::env::var("SOROBAN_FORGE_NO_UPDATE_CHECK")
        .map(|v| matches!(v.as_str(), "1" | "true" | "yes"))
        .unwrap_or(false)
    {
        return false;
    }
    // Persistent forge.toml opt-out.  We do a best-effort discovery from the
    // current directory; errors (no file, parse failures) default to enabled.
    if let Ok(cwd) = std::env::current_dir() {
        if let Ok(Some(config)) = soroban_forge_core::ForgeConfig::load_from(&cwd) {
            if let Some(false) = config.defaults.update_check {
                return false;
            }
        }
    }
    true
}
