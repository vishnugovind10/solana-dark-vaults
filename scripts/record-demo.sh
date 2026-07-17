#!/usr/bin/env bash
set -euo pipefail

if ! command -v asciinema >/dev/null 2>&1; then
  echo "missing required command: asciinema" >&2
  exit 1
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_path="${1:-$repo_root/demo.cast}"

mkdir -p "$(dirname "$output_path")"
asciinema rec --overwrite -c "$repo_root/scripts/demo.sh" "$output_path"

echo "recorded: $output_path"
echo "convert with: agg '$output_path' demo.gif"
echo "or publish the cast with an asciinema-player embed"
