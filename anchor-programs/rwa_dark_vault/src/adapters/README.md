# Pool adapter contract

`PoolAdapter` exposes `deposit`, `withdraw`, and `read_position`. The shipped `MockPool` is deterministic and performs no CPI.

A real Kamino or JupLend adapter must:

1. Pin and verify the target program ID.
2. Constrain reserve, market, vault-authority, and token accounts in the Anchor context.
3. Derive CPI accounts from canonical protocol state rather than caller-provided unchecked addresses.
4. Enforce minimum received amounts and explicit slippage limits.
5. Normalize Token and Token-2022 decimals before updating vault accounting.
6. Add local-validator tests against pinned protocol artifacts and document upgrade-authority risk.

No real adapter belongs in the registry until those checks are executable.

