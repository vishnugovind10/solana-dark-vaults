# Known limitations

- Arcium does not publish a native Windows CLI. The local MXE cluster therefore cannot be verified on the machine used for the initial release.
- The portable demo uses the same deterministic allocation rules and a local X25519/AES-GCM transport, but it is not evidence of execution by an Arcium MXE.
- Token-2022 confidential transfer support is feature-gated design work. Local custody is represented by accounting state; no real-value token CPI should be inferred.
- Pool adapters are deterministic mocks. There are no Kamino, JupLend, or Maple write integrations.
- Settlement is public. Realized allocations and timing remain observable after execution.

