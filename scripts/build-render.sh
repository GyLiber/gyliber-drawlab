#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
# Render can inherit a read-only system Rust installation. Keep all mutable
# toolchains, downloads and installed tools in the writable checkout instead.
export RUSTUP_HOME="$PWD/.drawlab-build/rustup"
export CARGO_HOME="$PWD/.drawlab-build/cargo"
mkdir -p "$RUSTUP_HOME" "$CARGO_HOME"
export PATH="$CARGO_HOME/bin:$PATH"
if [[ "$(node --version)" != "v24.19.0" ]]; then
  echo "Build requires Node 24.19.0" >&2
  exit 1
fi
if ! command -v rustup >/dev/null 2>&1; then
  installer="$(mktemp)"
  trap 'rm -f "$installer"' EXIT
  curl --proto '=https' --tlsv1.2 --fail --silent --show-error https://sh.rustup.rs -o "$installer"
  sh "$installer" -y --profile minimal --default-toolchain 1.98.1 --no-modify-path
fi
rustup toolchain install 1.98.1 --profile minimal --target wasm32-unknown-unknown
if [[ "$(wasm-bindgen --version 2>/dev/null || true)" != "wasm-bindgen 0.2.104" ]]; then
  cargo install wasm-bindgen-cli --version 0.2.104 --locked
fi
npm --prefix web ci --ignore-scripts --include=dev
bash scripts/build-web.sh
