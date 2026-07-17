# July 2026 adversarial self-audit

Scope: `anchor-programs/rwa_dark_vault` at the v0.1.x hardening commit. Method: manual walk-through of every instruction context and state transition using `checklist.md`, followed by host regression tests.

This is a self-review of unaudited reference code. It is not a third-party audit, certification, or deployment recommendation.

## Result summary

| ID | Severity | Result |
|---|---|---|
| SDV-001 | High | Open design finding; issue-ready below |
| SDV-002 | High | Open design finding; issue-ready below |
| SDV-003 | Medium | Open design finding; issue-ready below |
| SDV-004 | Low | Open lifecycle finding; issue-ready below |
| SDV-005 | Low | Fixed in this pass; regression test added |

## SDV-001 — Track depositor shares and enforce custody

**Severity:** High if the program were connected to assets  
**Location:** `instructions/deposit.rs:5-14`, `instructions/withdraw.rs:5-14`, `state.rs:44-90`  
**Description:** Any signer can increase aggregate assets and shares, and any signer can burn aggregate shares. The program stores no depositor share account, verifies no token account owner, and performs no token transfer. The current behavior is acceptable only for the explicitly mocked accounting boundary; attaching custody would allow unauthorized withdrawals.  
**Suggested fix:** Add a canonical per-owner share PDA or mint real share tokens, constrain owner token accounts, perform checked Token/Token-2022 CPIs, and add cross-user authorization tests before enabling custody.

## SDV-002 — Bind settlement to verified Arcium callback accounts

**Severity:** High  
**Location:** `instructions/settle_rebalance.rs:5-23`, `state.rs:111-136`  
**Description:** Settlement authenticates a configured signer and epoch but does not validate an Arcium computation account, output commitment, callback account set, or proof of execution. A compromised callback key can choose any weights that sum to 10,000 bps.  
**Suggested fix:** Replace the standalone signer boundary with the current Arcium callback macro/account model, bind the pending commitment and computation identifier, validate all Arcium-owned accounts, and add spoofed callback and replay integration tests.

## SDV-003 — Validate mint and Token-2022 custody assumptions

**Severity:** Medium in the current reference; High if custody is added without constraints  
**Location:** `instructions/initialize.rs:7-19`  
**Description:** `asset_mint` is an unchecked account whose address is only stored. The program does not verify Token or Token-2022 ownership, mint extensions, decimals, transfer-hook behavior, or confidential-transfer configuration.  
**Suggested fix:** Implement the custody adapter first, accept an interface account owned by an allowlisted token program, validate supported extensions and decimals, and add local-validator tests for both accepted and rejected mint configurations.

## SDV-004 — Add explicit account and authority lifecycle

**Severity:** Low  
**Location:** `instructions/admin.rs:5-14`, `instructions/mod.rs:1-13`  
**Description:** The authority can pause but cannot rotate authority or callback authority, cancel a stale pending computation, or close an empty vault to reclaim rent. Permanent key loss or an abandoned pending computation has no explicit recovery path.  
**Suggested fix:** Define narrowly scoped rotation, pending-cancel, and close instructions with timelock or two-step acceptance as appropriate; require zero assets/shares and no pending computation before close; cover every transition with authorization tests.

## SDV-005 — Reject an empty computation commitment

**Severity:** Low  
**Location:** `state.rs:92-108`, `state.rs:221-228`  
**Description:** The queue previously accepted `[0; 32]`, which cannot serve as a meaningful binding to encrypted inputs.  
**Resolution:** Added `InvalidCommitment`, rejected the all-zero value before mutating pending state, and added a regression test proving state remains unchanged.

## Pass evidence

- Vault ownership is enforced by Anchor's `Account<Vault>` deserialization in all mutation contexts.
- Initialization derives a canonical vault PDA from `VAULT_SEED` and the asset mint (`instructions/initialize.rs:7-18`).
- Authority-only pause and queue paths use `has_one = authority` (`instructions/admin.rs:5-10`, `instructions/queue_rebalance.rs:5-10`).
- Share calculations use checked `u128` multiplication, floor division, and checked `u64` state updates (`state.rs:44-90`).
- Settlement rejects missing or mismatched pending epochs, weights above 10,000 bps, and sums other than exactly 10,000 bps (`state.rs:111-136`).
- Pause checks cover every non-admin state mutation and have an executable host test (`state.rs:39-42`, `192-199`).
