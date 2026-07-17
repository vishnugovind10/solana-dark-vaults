# Anchor self-audit checklist

Use this checklist for every change to `rwa_dark_vault`.

- [x] Signer and owner checks per instruction: initialization and admin paths constrain signers; `Account<Vault>` enforces program ownership. Deposit and withdrawal lack per-user ownership state — finding SDV-001.
- [x] PDA seed canonicality: initialization uses `b"vault"`, the asset mint, and Anchor's canonical bump (`instructions/initialize.rs:7-18`).
- [x] Checked arithmetic and rounding direction: deposits and withdrawals use `u128` intermediates, checked state mutation, and floor rounding (`state.rs:44-90`).
- [x] Callback-authorization spoofing surface: the configured signer is checked, but no Arcium computation account or callback proof is bound — finding SDV-002.
- [x] Pause-state coverage: deposits, withdrawals, queue, and settlement call `assert_active`; host tests exercise pause/unpause (`state.rs:39-42`, `45-46`, `71-72`, `92-94`, `111-112`, `192-199`).
- [x] Account-close and rent paths: no close or authority-rotation instruction exists — finding SDV-004.
- [x] Token-2022 assumptions: the mint is deliberately unchecked and no custody CPI exists — finding SDV-003.
- [x] Computation input integrity: all-zero commitments are rejected and covered by a regression test (`state.rs:92-108`, `221-228`).

The completed July 2026 walk-through and issue-ready findings are in `2026-07-self-audit.md`.
