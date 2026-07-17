#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
extracted="$(mktemp)"
trap 'rm -f "$extracted"' EXIT

# Git Bash ships a GNU `link.exe` that shadows MSVC's linker. Preserve native Rust verification
# when this script is launched from a Visual Studio developer environment.
if [[ "${OSTYPE:-}" == msys* && -n "${VCToolsInstallDir:-}" ]] && command -v cygpath >/dev/null; then
  export PATH="$(cygpath -u "${VCToolsInstallDir}bin/Hostx64/x64"):$PATH"
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
