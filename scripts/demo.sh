#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
npm --prefix "$repo_root/agent-coordinator" ci --no-audit --no-fund
npm --prefix "$repo_root/agent-coordinator" run demo

