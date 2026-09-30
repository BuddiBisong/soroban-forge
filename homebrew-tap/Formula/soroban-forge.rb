# typed: false
# frozen_string_literal: true

# Homebrew formula for soroban-forge.
# Hosted in the tap repo: https://github.com/soroban-forge-labs/homebrew-tap
#
# To publish a new release:
#   1. Upload the release archives to GitHub Releases for the new tag.
#   2. Run `brew bump-formula-pr` or update `version`, `url`, and `sha256`
#      fields below and open a PR against this tap repository.
class SorobanForge < Formula
  desc "Scaffolding, test-harness and CI toolkit for Soroban smart contracts on Stellar"
  homepage "https://github.com/soroban-forge-labs/soroban-forge"
  version "0.1.0"
  license "Apache-2.0"

  # ---------------------------------------------------------------------------
  # Release archives — one per supported platform.
  # Each archive must be a .tar.gz containing a single `soroban-forge` binary.
  # SHA-256 values are filled in by the release workflow (or `brew bump-formula-pr`).
  # ---------------------------------------------------------------------------

  on_macos do
    on_arm do
      url "https://github.com/soroban-forge-labs/soroban-forge/releases/download/v#{version}/soroban-forge-#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_AARCH64_APPLE_DARWIN"
    end

    on_intel do
      url "https://github.com/soroban-forge-labs/soroban-forge/releases/download/v#{version}/soroban-forge-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_X86_64_APPLE_DARWIN"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/soroban-forge-labs/soroban-forge/releases/download/v#{version}/soroban-forge-#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_SHA256_AARCH64_LINUX"
    end

    on_intel do
      url "https://github.com/soroban-forge-labs/soroban-forge/releases/download/v#{version}/soroban-forge-#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_SHA256_X86_64_LINUX"
    end
  end

  # No build step — the archive already contains the pre-compiled binary.
  bottle :unneeded

  def install
    bin.install "soroban-forge"
  end

  # Wire up shell completion for bash, zsh, and fish.
  def caveats
    <<~EOS
      Shell completions are available. To enable them run:

        # bash
        soroban-forge completions bash > $(brew --prefix)/etc/bash_completion.d/soroban-forge

        # zsh
        soroban-forge completions zsh > $(brew --prefix)/share/zsh/site-functions/_soroban-forge

        # fish
        soroban-forge completions fish > ~/.config/fish/completions/soroban-forge.fish

      Then restart your shell or source the relevant file.
    EOS
  end

  test do
    # Smoke-test: the binary must print a version string.
    assert_match version.to_s, shell_output("#{bin}/soroban-forge --version")
  end
end
