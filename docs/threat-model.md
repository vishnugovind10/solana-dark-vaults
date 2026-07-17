# Threat model

## Scope

The protected asset is pre-settlement strategy intent: yield/risk inputs, target weights, and intermediate allocation state. The design does not hide the existence of the vault, its program interactions, or final settlement.

| Data / event | Visibility | Notes |
|---|---|---|
| Deposits / withdrawals into vault | Public; amounts may be maskable with a future C-SPL path | Aggregate flows and account interactions remain observable |
| Strategy logic | Code public; runtime inputs and intermediate state confidential in the Arcis design | A private fork could keep circuit logic proprietary; this repository does not |
| Yield/risk inputs, target weights | Confidential in transit and compute when using the MXE path | Client encrypts through an X25519-derived shared secret |
| Rebalance intent before trade | Absent from the normal public mempool in the intended MXE path | This is the narrow MEV-mitigation boundary |
| Settlement CPIs | Public after submission | Realized allocation can be inferred |
| Agent payment flows | Public in a real x402 integration | The repository uses an in-process mock and moves no funds |

## Adversaries

### Mempool searcher

Can observe queue transactions, settlement transactions, vault balances, accounts, cadence, priority fees, and pool CPIs. Cannot read encrypted yield/risk inputs or target weights from a correctly formed MXE submission. Can react to the public settlement transaction and subsequent state.

### Colluding MXE nodes

The configured Cerberus backend is designed for a dishonest majority. Confidentiality and correct output require at least one honest node; authenticated shares permit detection and abort on malicious behavior. All nodes colluding defeats the assumption. Any subset can threaten liveness by going offline or forcing aborts.

### Malicious oracle feeder

Can supply validly encoded but economically false yields or risk scores. Encryption does not establish data truth. Production work needs authenticated sources, freshness bounds, quorum or median rules, and an on-chain commitment policy. The shipped source is explicitly deterministic mock data.

### Malicious coordinator

Can withhold epochs, choose timing, replay external requests, submit malformed ciphertext, or refuse settlement. It cannot sign as the configured callback authority or bypass Anchor account constraints. Current recovery is intentionally fail-closed: a persisted pending computation requires manual reconciliation.

## Token confidentiality boundary

Token-2022 confidential transfer and C-SPL are not anonymity systems. Amount masking does not hide counterparties, program IDs, instruction structure, account graph, or timing. This release does not execute the C-SPL path and makes no token-custody confidentiality claim.

## What an attacker still learns

- The vault program, asset mint, registry accounts, and transaction graph.
- Deposit and withdrawal timing; amounts unless an independently verified confidential-token path masks them.
- That a rebalance was queued, when it settled, which pools received CPIs, and the realized post-trade positions.
- Repeated cadence, priority fee, compute usage, and failure patterns that may correlate with strategy conditions.
- Coordinator x402 counterparties, payment timing, and amounts in a real on-chain payment implementation.
- The complete open-source allocation function. Only inputs and intermediate runtime state are intended to be confidential.

## Residual risks

- Post-trade inference from balance deltas and pool interactions.
- Timing correlation between oracle updates, queued commitments, and settlement cadence.
- Oracle manipulation and stale data.
- Denial of service by the coordinator, callback signer, Solana congestion, or MXE nodes.
- Callback-key compromise, program-upgrade authority compromise, or an incorrect pool adapter.
- Integer rounding that is safe against overdraft but may create dust or favor remaining shareholders.
- Toolchain drift in mainnet-alpha dependencies and cluster configuration.

## Out of scope

Custody governance, legal eligibility, sanctions controls, price-oracle design, formal verification, operational key management, and a withdrawal queue are not implemented. No value should be deployed against this code.

