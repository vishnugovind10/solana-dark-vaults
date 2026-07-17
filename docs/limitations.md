# Known limitations

- Arcium does not publish a native Windows CLI. The real MXE build and local-cluster path run only in hosted Linux CI or a contributor's Linux/WSL2 environment.
- Anchor 1.1.2 and Arcis 0.13.2 resolve incompatible `digest` 0.11 dependency lines in the workspace layout required by `arcium build`. The repository therefore pins the official Arcium prerequisite Anchor 1.0.2 instead of Anchor 1.1.2.
- `GHSA-w5hq-g745-h8pq` is resolved in the lockfile with a scoped `jayson > uuid@11.1.1` override while `@arcium-hq/client` remains pinned at 0.13.2. The affected dependency resolved to `uuid@8.3.2`, for which no patched 8.x release exists, so the override crosses to the lowest patched major. Source triage found only parameterless `uuid.v4()` calls and no caller-supplied buffer passed to `uuid.v3()`, `uuid.v5()`, or `uuid.v6()`.
- The portable demo uses the same deterministic allocation rules and a local X25519/AES-GCM transport, but it is not evidence of execution by an Arcium MXE.
- Token-2022 confidential transfer support is feature-gated design work. Local custody is represented by accounting state; no real-value token CPI should be inferred.
- Pool adapters are deterministic mocks. There are no Kamino, JupLend, or Maple write integrations.
- Settlement is public. Realized allocations and timing remain observable after execution.
- Devnet deployment is preparation-only: the guarded script matches the documented `arcium deploy` parameters, but the program does not yet expose Arcium computation-definition and verified callback accounts, so no confidential devnet round can be run or claimed.
