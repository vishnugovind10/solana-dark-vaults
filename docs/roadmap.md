# Roadmap

Dates are targets, not release promises.

## v0.1 — July 2026

- Checked vault accounting and settlement invariants.
- Fixed-three-pool allocator with Arcis source and cleartext equivalence corpus.
- Local encrypted coordinator simulator, mock x402 ledger, and portable demo.
- Threat model, contribution infrastructure, and pinned versions.

## v0.2 — target Q3 2026

- Compile and test the Arcis instruction on a two-node Linux local cluster.
- Wire Arcium computation-definition and callback accounts into the Anchor program.
- Add one read-only Kamino or JupLend data adapter with source authentication and freshness metadata.
- Deploy an MXE to devnet cluster offset `456` only after rechecking current official configuration.

## v0.3 — target Q4 2026

- Design an epoch-boundary withdrawal queue with explicit fairness and dust rules.
- Evaluate settlement timing decorrelation and its liveness/cost trade-offs.
- Add property tests and fuzzing for share math, callback replay protection, and adapter account validation.
- Prototype one write adapter against pinned local-validator artifacts; no mainnet capital.

