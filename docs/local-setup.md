# Local setup

## Portable verification path

Install Git, Rust 1.97.1 through rustup, and Node.js 24 LTS. Then run the commands from the repository root:

```text
cargo test --workspace
npm --prefix agent-coordinator ci
npm --prefix agent-coordinator run check
npm --prefix agent-coordinator test
```

Windows users can run `powershell -File scripts/demo.ps1`; Linux and macOS users can run `./scripts/demo.sh` after Phase 3.

## Arcium local-cluster path

Arcium currently publishes its quick installer for Linux and macOS, not native Windows. On Ubuntu, WSL, or macOS:

```text
curl --proto '=https' --tlsv1.2 -sSfL https://install.arcium.com/ | bash
arcium --version
solana --version
anchor --version
```

The current official prerequisites are Solana CLI 3.1.10, Anchor 1.0.2 or later compatible tooling, Yarn, Docker, and Docker Compose. This repository pins Anchor crates at 1.1.2 and Arcium packages at 0.13.2. See `docs/limitations.md` before interpreting the circuit integration status.

