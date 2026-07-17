# Devnet deployment preparation

No devnet deployment or confidential computation has been performed for this release. The script is a guarded preparation artifact and must be run only from Linux or WSL2 after the hosted/local Arcium path is green.

## Prerequisites

- Arcium 0.13.2, Solana CLI 3.1.10, Anchor 1.1.2, and Docker.
- A funded Solana devnet keypair with roughly 2–5 devnet SOL.
- A dedicated HTTPS devnet RPC endpoint.
- A completed `arcium build` and `arcium test` on the same commit.

Set `DEVNET_KEYPAIR` to the keypair JSON path and `DEVNET_RPC_URL` to the dedicated endpoint. Optional `ARCIUM_RECOVERY_SET_SIZE` defaults to the documented minimum of `4`; `ARCIUM_CLUSTER_OFFSET` defaults to devnet offset `456` and the script rejects any other value. Run `./scripts/deploy-devnet.sh`, inspect the printed program, MXE, cluster, and recovery-set plan, then type `y` only if every value is correct.

The script executes the parameter-complete form of `arcium deploy`. It creates or updates the upgradeable program and ProgramData accounts, initializes the program's MXE account, and establishes its cluster/recovery state. It does not initialize a computation definition; current Arcium documentation treats that as a separate post-deployment instruction.

## One confidential computation round

This step is intentionally blocked in v0.1.0. The current Anchor program does not expose the Arcium-generated computation-definition, queue, and verified callback account contexts identified in finding SDV-002 and GitHub issue #16. Running `arcium test --cluster devnet` today would execute the portable Rust script configured in `Anchor.toml`, not an on-chain confidential computation, so it must not be presented as one.

After issue #16 is resolved, the devnet evidence procedure is:

1. Initialize `compute_allocation`'s computation-definition account once and record that transaction signature.
2. Encrypt a fixed fixture, submit it with a unique computation offset, and record the queue transaction signature.
3. Wait for Arcium finalization, verify the signed callback against the computation and cluster accounts, and record the finalization/callback transaction signature.
4. Decrypt the returned weights and compare them to the matching fixture in `tests/fixtures/allocation-corpus.json`.
5. Add the Solana Explorer links, commit SHA, cluster offset, and exact tool versions below.

## Verified devnet computation

Status: **not run**

- Program transaction: TODO
- Computation-definition transaction: TODO
- Queue transaction: TODO
- Finalization/callback transaction: TODO
- Commit: TODO
- Observed output: TODO
