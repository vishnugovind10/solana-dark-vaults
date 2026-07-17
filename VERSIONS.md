# Toolchain matrix

Last verified: 2026-07-17.

| Component | Pinned version | Verification status |
|---|---:|---|
| Rust | 1.97.1 | Verified on Windows 11 |
| Node.js | 24.x LTS | Verified with 24.11.1 on Windows 11; hosted Arcium job pins 24.10.0 |
| TypeScript | 7.0.2 | Verified |
| Solana CLI / Agave | 3.1.10 | Required by current Arcium install docs; Linux/WSL only for this repository |
| Anchor CLI / crates | 1.1.2 | Crate verified; CLI requires Linux/WSL for the full validator path |
| Arcium CLI / Arx / Arcis / TS SDK | 0.13.2 | Packages pinned; hosted Linux CLI verification pending |

The repository's portable checks (`cargo test`, coordinator typecheck/tests, and the deterministic demo) remain the baseline CI gate. The separate hosted Linux job installs the versions above and records the Arcium build and local-cluster result.
