#!/usr/bin/env bash
# Called only by the main-branch CI publish job, after verify and audit succeed.
set -euo pipefail
cd "$(dirname "$0")/.."
[[ "$GITHUB_EVENT_NAME" == push && "$GITHUB_REF" == refs/heads/main ]]
if [[ ! -f release-intent.json ]]; then
  echo "No reviewed release intent; publication skipped."
  exit 0
fi
release_version="$(node -p 'JSON.parse(require("fs").readFileSync("web/package.json","utf8")).version')"
release_tag="v${release_version}"
node --input-type=module - "$release_version" <<'JS'
import fs from 'node:fs';
const intent=JSON.parse(fs.readFileSync('release-intent.json','utf8'));
if(intent.version!==process.argv[2] || intent.prerelease!==true || !/^[0-9a-f]{40}$/.test(intent.deployed_commit)) throw Error('Invalid release intent');
JS
if gh release view "$release_tag" --repo "$GITHUB_REPOSITORY" --json isDraft --jq '.isDraft' > /tmp/drawlab-release-state; then
  [[ "$(cat /tmp/drawlab-release-state)" == false ]] || { echo "Existing draft needs review" >&2; exit 1; }
  echo "Release already published; immutable history retained."
  exit 0
fi
curl --proto '=https' --tlsv1.2 --fail --silent --show-error --max-time 30 https://gyliber-drawlab.onrender.com/build-info.json -o /tmp/drawlab-live-identity.json
node --input-type=module <<'JS'
import fs from 'node:fs';
const intent=JSON.parse(fs.readFileSync('release-intent.json','utf8'));
const live=JSON.parse(fs.readFileSync('/tmp/drawlab-live-identity.json','utf8'));
if(live.version!==intent.version || live.commit!==intent.deployed_commit || live.scope!=='laboratory-preview') throw Error('Deployment does not match approved intent');
JS
gh run download "$GITHUB_RUN_ID" --repo "$GITHUB_REPOSITORY" --name drawlab-release-bundle --dir release-assets
(cd release-assets && sha256sum -c SHA256SUMS)
gh release create "$release_tag" release-assets/* --repo "$GITHUB_REPOSITORY" --target "$GITHUB_SHA" --title "DrawLab ${release_version} — exact probability analysis" --notes-file "docs/releases/${release_tag}.md" --prerelease --draft
gh release edit "$release_tag" --repo "$GITHUB_REPOSITORY" --draft=false --prerelease --latest=false
