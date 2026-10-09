#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

# Render's preinstalled rustup and its sibling Cargo shims may live on a
# read-only filesystem. Redirecting RUSTUP_HOME alone does not make those
# executables writable: rustup updates proxy executables next to its own binary.
# Bootstrap a separate rustup inside the writable checkout, regardless of
# whether an inherited rustup appears on PATH.
export RUSTUP_HOME="$PWD/.drawlab-build/rustup"
export CARGO_HOME="$PWD/.drawlab-build/cargo"
mkdir -p "$RUSTUP_HOME" "$CARGO_HOME"
export PATH="$CARGO_HOME/bin:$PATH"

if [[ "$(node --version)" != "v24.19.0" ]]; then
  echo "Build requires Node 24.19.0" >&2
  exit 1
fi

if [[ ! -x "$CARGO_HOME/bin/rustup" ]]; then
  installer="$(mktemp)"
  trap 'rm -f "$installer"' EXIT
  curl --proto '=https' --tlsv1.2 --fail --silent --show-error https://sh.rustup.rs -o "$installer"
  sh "$installer" -y --profile minimal --default-toolchain none --no-modify-path
fi

if [[ "$(command -v rustup)" != "$CARGO_HOME/bin/rustup" ]]; then
  echo "Refusing to use Rust tools outside writable CARGO_HOME" >&2
  exit 1
fi

rustup toolchain install 1.98.1 --profile minimal --target wasm32-unknown-unknown
if [[ "$(wasm-bindgen --version 2>/dev/null || true)" != "wasm-bindgen 0.2.104" ]]; then
  cargo install wasm-bindgen-cli --version 0.2.104 --locked
fi
npm --prefix web ci --ignore-scripts --include=dev
bash scripts/build-web.sh
