#!/usr/bin/env bash
set -euo pipefail

readonly RUST_VERSION="1.97.1"
readonly NODE_VERSION="24.10.0"
readonly SOLANA_VERSION="3.1.10"
readonly ANCHOR_VERSION="1.1.2"
readonly ARCIUM_VERSION="0.13.2"

sudo apt-get update
sudo apt-get install -y --no-install-recommends build-essential curl libssl-dev libudev-dev pkg-config xz-utils

if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "$RUST_VERSION"
fi
export PATH="$HOME/.cargo/bin:$PATH"
rustup toolchain install "$RUST_VERSION"
rustup default "$RUST_VERSION"

curl --proto '=https' --tlsv1.2 -sSfL "https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-x64.tar.xz" -o /tmp/node.tar.xz
sudo tar -xJf /tmp/node.tar.xz --strip-components=1 -C /usr/local

sh -c "$(curl --proto '=https' --tlsv1.2 -sSfL https://release.anza.xyz/v${SOLANA_VERSION}/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
solana-keygen new --silent --no-bip39-passphrase --force

cargo install --git https://github.com/solana-foundation/anchor --tag "v${ANCHOR_VERSION}" anchor-cli --locked --force
curl --proto '=https' --tlsv1.2 -sSfL "https://bin.arcium.com/download/arcup_x86_64_linux_${ARCIUM_VERSION}" -o "$HOME/.cargo/bin/arcup"
chmod +x "$HOME/.cargo/bin/arcup"
arcup install "$ARCIUM_VERSION"

rustc --version
node --version
solana --version
anchor --version
arcium --version
