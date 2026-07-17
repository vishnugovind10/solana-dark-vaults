# Contributing

Contributions should make a claim more testable, reduce a documented limitation, or strengthen a trust boundary.

## Setup

Follow `docs/local-setup.md`. Before opening a pull request, run the README verification block and `npm --prefix agent-coordinator run coverage`. Arcium-facing changes also require `arcium build && arcium test` on Linux or macOS; attach the versions and command output to the pull request. The Anchor and Arcis packages use separate Cargo workspaces because their pinned dependency graphs currently select incompatible `digest` 0.11 release lines.

## Workflow

1. Open or select a scoped issue.
2. Branch from `main` using `feat/<short-name>`, `fix/<short-name>`, or `docs/<short-name>`.
3. Use conventional commits such as `feat(circuits): add cap boundary fixtures`.
4. Add tests for every changed invariant and update the implementation-status table if a boundary moves.
5. Keep mocks labeled in filenames, module docs, tests, and README status.
6. Request review only after CI passes and the pull-request checklist is complete.

Reviewers check account constraints, arithmetic, replay behavior, fixture validity, and whether documentation matches executable behavior. A new external dependency needs a concrete reason and a lockfile update.

## Good contribution areas

- **Accounting hardening:** property tests, dust analysis, deposit/withdraw sequence fuzzing.
- **Circuits:** additional fixtures, equivalent allocation strategies, fixed-width scaling analysis.
- **Adapters:** read-only data sources, protocol account maps, pinned local-validator artifacts.
- **Coordinator:** recovery tooling, authenticated oracle envelopes, x402 interoperability tests.
- **Documentation:** commands verified on clean Linux/macOS environments and threat-model corrections.

Do not add mainnet write paths, real-value deployment instructions, unsupported performance claims, or hidden network calls.
