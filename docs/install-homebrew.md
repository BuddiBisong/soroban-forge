# Installing via Homebrew

`soroban-forge` is available through an official [Homebrew](https://brew.sh)
tap.  The tap installs a **pre-compiled binary** — no Rust toolchain is
required on your machine.

## Requirements

- macOS or Linux
- [Homebrew](https://brew.sh) installed

## Install

```sh
brew tap soroban-forge-labs/tap
brew install soroban-forge
```

Or as a single command:

```sh
brew install soroban-forge-labs/tap/soroban-forge
```

Verify the installation:

```sh
soroban-forge --version
```

## Upgrade

```sh
brew upgrade soroban-forge
```

## Uninstall

```sh
brew uninstall soroban-forge
brew untap soroban-forge-labs/tap   # optional: remove the tap itself
```

## Shell completions

The binary ships with completion scripts for bash, zsh, and fish.  After
installing, run the relevant snippet once:

```sh
# bash
soroban-forge completions bash > $(brew --prefix)/etc/bash_completion.d/soroban-forge

# zsh
soroban-forge completions zsh > $(brew --prefix)/share/zsh/site-functions/_soroban-forge

# fish
soroban-forge completions fish > ~/.config/fish/completions/soroban-forge.fish
```

Then restart your shell (or `source` the file).

## How the binary is fetched

Each `brew install` pulls a release archive directly from
[GitHub Releases](https://github.com/soroban-forge-labs/soroban-forge/releases):

```
https://github.com/soroban-forge-labs/soroban-forge/releases/download/v<VERSION>/soroban-forge-<VERSION>-<TARGET>.tar.gz
```

Supported targets:

| Platform | Architecture | Target triple |
|----------|-------------|---------------|
| macOS    | Apple Silicon | `aarch64-apple-darwin` |
| macOS    | Intel        | `x86_64-apple-darwin`  |
| Linux    | arm64        | `aarch64-unknown-linux-gnu` |
| Linux    | x86_64       | `x86_64-unknown-linux-gnu`  |

## Alternative installation methods

| Method | Command |
|--------|---------|
| **cargo-binstall** (fetches release binary) | `cargo binstall soroban-forge` |
| **From source** | `cargo install --git https://github.com/soroban-forge-labs/soroban-forge` |
| **Source (local clone)** | `cargo install --path .` |

> **Note:** `cargo install` compiles from source and requires Rust ≥ 1.84.
> The Homebrew and `cargo-binstall` methods download pre-built binaries and
> have no Rust requirement.
