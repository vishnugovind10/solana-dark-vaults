# Local setup

## Portable verification path

Install Git, Rust 1.97.1 through rustup, and Node.js 24 LTS. Then run the commands from the repository root:

```text
cargo test --workspace --exclude encrypted-ixs
cargo test --manifest-path arcium-circuits/Cargo.toml
npm --prefix agent-coordinator ci
npm --prefix agent-coordinator run check
npm --prefix agent-coordinator test
```

Windows users can run `powershell -File scripts/demo.ps1`; Linux and macOS users can run `./scripts/demo.sh` after Phase 3.

## WSL2 setup from Windows

Run `wsl --install -d Ubuntu-24.04` in an elevated PowerShell terminal, restart Windows if prompted, and open Ubuntu. Inside the WSL2 shell, enable systemd by writing `[boot]` and `systemd=true` to `/etc/wsl.conf`, then run `wsl --shutdown` from PowerShell and reopen Ubuntu. Install Docker Engine using Docker's Ubuntu instructions and confirm `docker info` succeeds.

Clone this repository inside the Linux filesystem, open it in a dev container, and let `.devcontainer/setup.sh` install Rust 1.97.1, Node 24.10.0, Solana 3.1.10, Anchor 1.0.2, and Arcium 0.13.2. From the repository root, the exact verification command is `./scripts/localnet-up.sh`; it refuses to continue if Docker, Solana, Anchor, Arcium, or the Docker daemon is unavailable, then executes `arcium test` against the two-node configuration in `Arcium.toml`.

## Arcium local-cluster path

Arcium currently publishes its quick installer for Linux and macOS, not native Windows. On Ubuntu, WSL, or macOS:

```text
curl --proto '=https' --tlsv1.2 -sSfL https://install.arcium.com/ | bash
arcium --version
solana --version
anchor --version
```

The current official prerequisites are Solana CLI 3.1.10, Anchor 1.0.2, Yarn, Docker, and Docker Compose. This repository pins Anchor crates and CLI at 1.0.2 and Arcium packages and CLI at 0.13.2. See `docs/limitations.md` before interpreting the circuit integration status.
