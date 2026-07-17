# Changelog

## 0.1.0 - unreleased

### Dependency hardening

- Resolved `GHSA-w5hq-g745-h8pq` with a scoped lockfile override and documented the upstream major-version constraint and call-path triage.
- Grouped weekly minor and patch dependency updates per ecosystem and labeled generated pull requests consistently.

### Hosted Arcium verification

- Added a pinned Ubuntu Arcium build and local-cluster workflow, the conventional encrypted-instruction layout, and reproducible WSL2/dev-container setup.

### Demo recording

- Added an asciinema recording hook, documented GIF conversion, and kept the README media embed inactive until an asset is committed.

### Phase 0 - scaffold and toolchain

- Added the Rust workspace, Anchor crate, circuit crate, and TypeScript coordinator package.
- Pinned the verified toolchain and documented the native-Windows Arcium limitation.
- Added baseline CI for portable checks.

### Phase 1 - public settlement layer

- Implemented checked share accounting, pause control, computation queues, and callback-authorized settlement.
- Added the pool adapter boundary and a labeled deterministic mock.
- Added exact rounding, underflow, pause, and settlement invariant tests.

### Phase 2 - confidential allocation circuit

- Added a fixed-three-pool risk-adjusted yield allocator with explicit cap enforcement.
- Added an Arcis 0.13.2 encrypted instruction definition and a portable cleartext reference.
- Added 24 generated fixtures plus cap, tie, dominance, and invalid-input tests.

### Phase 3 - autonomous coordinator

- Added deterministic oracle, encrypted local MXE boundary, settlement adapter, and crash-safe epoch state.
- Added a local x402 challenge/sign/retry model with an auditable cost ledger.
- Added end-to-end and idempotency tests plus a readable one-epoch demo.

### Phase 4 - verification and demo path

- Added Linux/macOS and PowerShell demo launchers.
- Added an explicit Arcium local-cluster gate and executable documentation checker.
- Added Rust linting, TypeScript coverage, coordinator tests, and the demo to CI.

### Phase 5 - documentation

- Added an evidence-first README with scope, status, comparison, coverage, and source links.
- Documented accounts, epoch state, callback flow, MPC assumptions, residual risks, and public leakage.
- Added a dated roadmap with explicit validation gates.

### Phase 6 - collaboration infrastructure

- Added Apache-2.0 licensing, conduct, contribution, security, issue, and pull-request policies.
- Added dependency update configuration and a repository social-preview asset.
- Prepared concrete launch issues for adapters, circuits, documentation, and hardening.
