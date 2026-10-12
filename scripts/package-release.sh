#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
release_version="$(node -p 'JSON.parse(require("fs").readFileSync("web/package.json","utf8")).version')"
[[ "$release_version" =~ ^[0-9]+\.[0-9]+\.[0-9]+-alpha\.[0-9]+$ ]]
cargo build -p drawlab-cli --release --locked
mkdir -p release-assets
tar -czf "release-assets/drawlab-${release_version}-web.tar.gz" -C web/dist .
tar -czf "release-assets/drawlab-${release_version}-linux-x86_64.tar.gz" -C target/release drawlab
cp "docs/releases/v${release_version}.md" release-assets/RELEASE-NOTES.md
cp LICENSE release-assets/LICENSE
(cd release-assets && sha256sum -- *.tar.gz RELEASE-NOTES.md LICENSE > SHA256SUMS && sha256sum -c SHA256SUMS)
