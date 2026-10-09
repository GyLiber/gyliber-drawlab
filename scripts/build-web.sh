#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
if [[ "$(wasm-bindgen --version)" != "wasm-bindgen 0.2.104" ]]; then
  echo "Install matching tool: cargo install wasm-bindgen-cli --version 0.2.104 --locked" >&2
  exit 1
fi
cargo build -p drawlab-wasm --target wasm32-unknown-unknown --release --locked
wasm-bindgen target/wasm32-unknown-unknown/release/drawlab_wasm.wasm --target web --out-dir web/pkg
npm --prefix web run build
node scripts/build-info.mjs
