$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
npm --prefix (Join-Path $repoRoot 'agent-coordinator') ci --no-audit --no-fund
npm --prefix (Join-Path $repoRoot 'agent-coordinator') run demo

