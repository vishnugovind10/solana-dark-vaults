## Change

<!-- What changed and why? -->

## Trust boundary

<!-- Which public/confidential boundary, account invariant, or mock status changes? -->

## Verification

- [ ] `cargo test --workspace --locked`
- [ ] `cargo test --manifest-path arcium-circuits/Cargo.toml --locked`
- [ ] `cargo clippy --workspace --all-targets --locked -- -D warnings`
- [ ] `npm --prefix agent-coordinator run check`
- [ ] `npm --prefix agent-coordinator test`
- [ ] Documentation commands pass
- [ ] New or changed mocks are labeled
- [ ] No secrets, generated state, wallets, or build output are included

## Arcium evidence, if applicable

<!-- Paste tool versions and the arcium build/test result. -->
