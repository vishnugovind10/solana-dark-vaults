#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
extracted="$(mktemp)"
trap 'rm -f "$extracted"' EXIT

# Git Bash ships a GNU `link.exe` that shadows MSVC's linker. Preserve native Rust verification
# when this script is launched from a Visual Studio developer environment.
if command -v cygpath >/dev/null && [[ "$(command -v link.exe 2>/dev/null || true)" == "/usr/bin/link.exe" ]]; then
  msvc_root="/c/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/VC/Tools/MSVC"
  if [[ -d "$msvc_root" ]]; then
    msvc_version="$(find "$msvc_root" -mindepth 1 -maxdepth 1 -type d | sort -V | tail -n 1)"
    export PATH="$msvc_version/bin/Hostx64/x64:$PATH"
  fi
fi

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
