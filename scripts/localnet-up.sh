#!/usr/bin/env bash
set -euo pipefail

for command_name in docker solana anchor arcium; do
  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "missing required command: $command_name" >&2
    exit 1
  fi
done

if ! docker info >/dev/null 2>&1; then
  echo "Docker is installed but the daemon is unavailable" >&2
  exit 1
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"
echo "arcium test starts the validator and two-node local cluster defined in Arcium.toml"
exec arcium test

