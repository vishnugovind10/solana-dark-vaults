#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
extracted="$(mktemp)"
trap 'rm -f "$extracted"' EXIT

awk '
  /^```bash$/ { in_block = 1; next }
  /^```$/ && in_block { in_block = 0; print ""; next }
  in_block { print }
' "$repo_root/README.md" "$repo_root"/docs/*.md > "$extracted"

if [[ ! -s "$extracted" ]]; then
  echo "no executable bash documentation blocks found" >&2
  exit 1
fi

cd "$repo_root"
bash -euo pipefail "$extracted"

