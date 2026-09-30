# soroban-forge Homebrew Tap

This repository is the official [Homebrew](https://brew.sh) tap for
[soroban-forge](https://github.com/soroban-forge-labs/soroban-forge).

It is intended to be published as a **separate GitHub repository** named
`soroban-forge-labs/homebrew-tap`.  The contents of this directory map
directly to the root of that repository — copy or symlink them when setting
the tap repo up.

## Install

```sh
brew tap soroban-forge-labs/tap
brew install soroban-forge
```

Or in a single command:

```sh
brew install soroban-forge-labs/tap/soroban-forge
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

## How it works

The formula in [`Formula/soroban-forge.rb`](Formula/soroban-forge.rb) downloads
a **pre-compiled binary** from the GitHub Releases page — no Rust toolchain is
required on the user's machine.

Supported platforms:

| Platform          | Architecture | Archive                                              |
|-------------------|--------------|------------------------------------------------------|
| macOS             | Apple Silicon (`arm64`) | `soroban-forge-<version>-aarch64-apple-darwin.tar.gz` |
| macOS             | Intel (`x86_64`) | `soroban-forge-<version>-x86_64-apple-darwin.tar.gz`  |
| Linux             | `arm64`       | `soroban-forge-<version>-aarch64-unknown-linux-gnu.tar.gz` |
| Linux             | `x86_64`      | `soroban-forge-<version>-x86_64-unknown-linux-gnu.tar.gz`  |

## Releasing a new version

1. Build and upload the four release archives to the GitHub Releases page for
   the new tag (e.g. `v0.2.0`).  The CI release workflow in the main
   repository does this automatically.
2. Compute each archive's SHA-256:
   ```sh
   shasum -a 256 soroban-forge-*.tar.gz
   ```
3. Update `version`, `url`, and `sha256` in
   [`Formula/soroban-forge.rb`](Formula/soroban-forge.rb).
4. Open a pull request against `soroban-forge-labs/homebrew-tap`.

Alternatively, use `brew bump-formula-pr` to automate steps 2–4:

```sh
brew bump-formula-pr \
  --url https://github.com/soroban-forge-labs/soroban-forge/releases/download/v<NEW_VERSION>/soroban-forge-<NEW_VERSION>-<TARGET>.tar.gz \
  --sha256 <SHA256> \
  soroban-forge-labs/tap/soroban-forge
```
