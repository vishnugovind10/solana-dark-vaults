# Toolchain matrix

Last verified: 2026-07-17.

| Component | Pinned version | Verification status |
|---|---:|---|
| Rust | 1.97.1 host toolchain; Anchor package metadata floor 1.89 | Host checks verified on Windows 11 and hosted Ubuntu; 1.89 metadata floor required by Solana BPF rustc |
| Node.js | 24.x LTS | Verified with 24.11.1 on Windows 11; hosted Arcium job pins 24.10.0 |
| TypeScript | 7.0.2 | Verified |
| Solana CLI / Agave | 3.1.10 | Required by current Arcium install docs; Linux/WSL only for this repository |
| Anchor CLI / crates | 1.0.2 | Official Arcium 0.13.x prerequisite; portable crate tests verified |
| Arcium CLI / Arx / Arcis / TS SDK | 0.13.2 | Verified in hosted Ubuntu Arcium workflow on commit `ecaca16`: devcontainer, `arcium build --skip-keys-sync`, and `arcium test` passed |

The repository's portable checks (`cargo test --workspace --exclude encrypted-ixs`, coordinator typecheck/tests, and the deterministic demo) remain the baseline CI gate. Arcis macros require the Arcium compiler environment, so the encrypted crate is built only by `arcium build`. The hosted Linux job installs the versions above and passed both Arcium build and local-cluster tests on commit `ecaca16`. Anchor 1.0.2 is intentionally pinned so the CLI can discover the program and `encrypted-ixs` in one Cargo workspace.
