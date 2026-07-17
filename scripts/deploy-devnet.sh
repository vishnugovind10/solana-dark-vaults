#!/usr/bin/env bash
set -euo pipefail

readonly DEVNET_CLUSTER_OFFSET="${ARCIUM_CLUSTER_OFFSET:-456}"
readonly RECOVERY_SET_SIZE="${ARCIUM_RECOVERY_SET_SIZE:-4}"

if [[ -z "${DEVNET_KEYPAIR:-}" ]]; then
  echo "DEVNET_KEYPAIR must point to a funded devnet keypair" >&2
  exit 1
fi
if [[ -z "${DEVNET_RPC_URL:-}" ]]; then
  echo "DEVNET_RPC_URL must be an explicit devnet RPC URL" >&2
  exit 1
fi
if [[ ! -f "$DEVNET_KEYPAIR" ]]; then
  echo "DEVNET_KEYPAIR does not exist: $DEVNET_KEYPAIR" >&2
  exit 1
fi
if [[ ! "$DEVNET_RPC_URL" =~ ^https:// ]]; then
  echo "DEVNET_RPC_URL must use https://" >&2
  exit 1
fi
if [[ "$DEVNET_CLUSTER_OFFSET" != "456" ]]; then
  echo "refusing non-devnet Arcium cluster offset: $DEVNET_CLUSTER_OFFSET" >&2
  exit 1
fi
if [[ ! "$RECOVERY_SET_SIZE" =~ ^[0-9]+$ ]] || (( RECOVERY_SET_SIZE < 4 )); then
  echo "ARCIUM_RECOVERY_SET_SIZE must be an integer of at least 4" >&2
  exit 1
fi

for command_name in arcium solana; do
  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "missing required command: $command_name" >&2
    exit 1
  fi
done

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

program_id="$(sed -n 's/^rwa_dark_vault = "\([^"]*\)"/\1/p' Anchor.toml)"
deployer="$(solana address --keypair "$DEVNET_KEYPAIR")"

cat <<EOF
Devnet deployment plan
  RPC: $DEVNET_RPC_URL
  payer/authority: $deployer
  program: rwa_dark_vault ($program_id)
  Arcium cluster offset: $DEVNET_CLUSTER_OFFSET
  recovery set size: $RECOVERY_SET_SIZE

arcium deploy will create or update:
  1. the upgradeable Solana program and ProgramData accounts for $program_id
  2. the MXE account derived by Arcium from $program_id
  3. MXE key-generation and recovery-set state for cluster offset $DEVNET_CLUSTER_OFFSET

It will not initialize computation-definition accounts; this repository does not yet expose
the required on-chain initialization instruction. No confidential round will be submitted.
EOF

read -r -p "Proceed with devnet deployment? [y/N] " reply
if [[ "$reply" != "y" && "$reply" != "Y" ]]; then
  echo "deployment cancelled"
  exit 0
fi

arcium deploy \
  --cluster-offset "$DEVNET_CLUSTER_OFFSET" \
  --recovery-set-size "$RECOVERY_SET_SIZE" \
  --keypair-path "$DEVNET_KEYPAIR" \
  --rpc-url "$DEVNET_RPC_URL"

echo "Deployment command completed. Preserve every transaction signature from the CLI output."
echo "Verify the program with: solana program show '$program_id' --url '$DEVNET_RPC_URL'"
echo "Do not claim a verified computation until a computation-definition transaction and finalization signature are recorded."
