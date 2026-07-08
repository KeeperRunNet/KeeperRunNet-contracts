# KeeperRunNet Contracts

The on-chain Soroban smart contracts powering the KeeperRunNet automation protocol on the Stellar network.

This workspace houses the core Rust contract logic, state definitions, on-chain execution guards, and the official in-depth protocol documentation. It acts as the decentralized source of truth for the entire KeeperRunNet ecosystem.

[![Network Deployments](https://img.shields.io/badge/Deployments-View%20All-orange)](/)
[![Documentation](https://img.shields.io/badge/Docs-Read%20Now-blue)](/docs)
[![Contributing](https://img.shields.io/badge/Contributing-Welcome-green)](/CONTRIBUTING.md)
[![License](https://img.shields.io/badge/License-MIT-purple)](LICENSE)

---

## Core Features

| Feature | Description |
|---------|-------------|
| **Job Registry** | Persistent on-chain storage tracking user automation configurations, targets, and execution states |
| **Execution Guards** | Strict validation checks ensuring jobs are only executed when trigger conditions are met and preventing double-executions |
| **Authorization** | Native Soroban `require_auth()` integration ensuring only job owners can register or cancel their automation pipelines |
| **Gas Optimized** | Minimal state footprint designed to keep registration and execution fees as low as possible for relayer nodes |

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (Edition 2021) |
| Blockchain | Stellar Network |
| Framework | Soroban SDK |
| Tooling | Soroban CLI |

---

## Available Contracts

The workspace compiles down to a single, highly optimized WebAssembly binary:

| Binary | Description |
|--------|-------------|
| `keeperrunnet_registry.wasm` | The main protocol entry point handling `register_job`, `execute_job`, and `cancel_job` |

---

## Project Structure

```text
contracts/
├── docs/
│   ├── architecture.md       # State machine and relayer interaction design
│   ├── security.md           # Known attack vectors and on-chain mitigations
│   └── gas_model.md          # Fee economics and reimbursement math
├── src/
│   ├── lib.rs                # Contract entry points and method definitions
│   ├── storage.rs            # Persistent and temporary storage keys
│   └── types.rs              # Custom structs and enums (JobConfig, TriggerCondition)
├── tests/
│   └── test_registry.rs      # End-to-end execution simulation tests
├── Cargo.toml                # Rust workspace dependencies
└── Makefile                  # Shortcut commands for building and deploying
```

---

## Getting Started

### Prerequisites

- Rust >= 1.75
- `wasm32-unknown-unknown` target installed
- Soroban CLI installed
- A funded Stellar testnet account configured in your CLI

### Installation and Build

```bash
# 1. Add the Wasm compilation target
rustup target add wasm32-unknown-unknown

# 2. Build the contract
cargo build --target wasm32-unknown-unknown --release

# 3. Optimize the Wasm binary (optional but recommended)
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/keeperrunnet_registry.wasm
```

### Testing

All smart contracts follow strict Test-Driven Development (TDD). Tests simulate a local ledger environment without requiring a live network connection.

```bash
# Run the full test suite
cargo test

# Run with output logging
cargo test -- --nocapture
```

### Deployment (Testnet)

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/keeperrunnet_registry.wasm \
  --source your-testnet-identity \
  --network testnet
```

> **Important:** Save the returned Contract ID. It is required by the backend relayer, the CLI, and all SDK implementations.

---

## Architecture Notes

**Trigger Evaluation** — To save user gas, complex condition evaluation (such as checking whether a specific block height has been reached) happens off-chain in the backend relayer. The contract only performs a final, lightweight sanity check (`env.ledger().sequence() >= target_height`) before invoking the target contract.

**Storage Footprint** — `Persistent` storage is used for `JobConfig` records to prevent accidental eviction. `Temporary` storage is used for short-lived execution lock states to minimize ledger footprint.

---

## Security

Smart contract security is a top priority for this protocol.

**Auth Verification** — `owner.require_auth()` is strictly enforced on all state-mutating operations. No external party can register, execute, or cancel a job on behalf of another address.

**Re-entrancy Protection** — State updates (`config.executed = true`) are committed to storage before any external `env.invoke_contract()` call is made, preventing re-entrancy attacks.

For the full threat model and mitigation strategies, see [docs/security.md](docs/security.md).

---

## Documentation

The `docs/` directory contains the full protocol specification:

| Document | Description |
|----------|-------------|
| [architecture.md](docs/architecture.md) | State machine design, relayer interaction flow, and contract lifecycle |
| [security.md](docs/security.md) | Full threat model, known attack vectors, and on-chain mitigations |
| [gas_model.md](docs/gas_model.md) | Fee economics, reimbursement math, and gas optimization strategies |

---

## GitHub Actions Workflows

All CI/CD pipelines live in `.github/workflows/`.

### Project Structure

```text
.github/
└── workflows/
    ├── ci.yml          # Runs on every pull request and push to main
    ├── deploy.yml      # Deploys the optimized Wasm binary to testnet or mainnet
    └── audit.yml       # Weekly automated security audit of Rust dependencies
```

### CI Pipeline (`ci.yml`)

Runs on every pull request and push to `main`. Ensures the contract builds, passes all tests, and meets formatting standards before any merge is allowed.



### Deploy Pipeline (`deploy.yml`)

Runs manually via `workflow_dispatch` or automatically when a release tag is pushed. Downloads the Wasm artifact from the CI build, optimizes it, and deploys to the target network.


### Security Audit Pipeline (`audit.yml`)

Runs on a weekly schedule to automatically scan all Rust dependencies for known vulnerabilities using `cargo audit`.

```yaml
name: Security Audit

on:
  schedule:
    - cron: '0 8 * * 1'  # Every Monday at 08:00 UTC
  workflow_dispatch:

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install cargo-audit
        run: cargo install cargo-audit
      - name: Run dependency audit
        run: cargo audit
```

> All sensitive values such as `STELLAR_SECRET_KEY` are stored as **GitHub Actions Secrets** and injected at runtime — never hardcoded in workflow files.

---

## Roadmap

### Planned

- [ ] `JobConfig` struct and `TriggerCondition` enum definitions
- [ ] `register_job` entry point with `require_auth()` enforcement
- [ ] `execute_job` entry point with block height sanity check
- [ ] `cancel_job` entry point with owner authorization guard
- [ ] Re-entrancy protection on all external contract invocations
- [ ] Persistent storage layout for job registry
- [ ] Temporary storage layout for execution lock states
- [ ] End-to-end integration tests using the Soroban test environment
- [ ] Wasm binary optimization pipeline via Soroban CLI
- [ ] Testnet deployment and Contract ID documentation
- [ ] `architecture.md` protocol specification
- [ ] `security.md` threat model documentation
- [ ] `gas_model.md` fee economics documentation

---

## Contributing

When contributing to the contracts, please ensure:

- All new methods include comprehensive inline Rustdoc comments
- Changes to Wasm inputs or outputs are reflected in the `docs/` folder
- `cargo test` and `cargo fmt` pass without warnings before opening a pull request

### Quick Start for Contributors

1. **Find an issue** — Check `good first issues` or `help wanted` on the issues board
2. **Read the guide** — See [CONTRIBUTING.md](CONTRIBUTING.md)
3. **Set up locally** — Follow the installation and build steps above
4. **Make your changes** — Create a feature branch
5. **Test** — Ensure `cargo test` and `cargo clippy` pass
6. **Submit a PR** — Open a pull request with a clear description

---

## Community and Support

- **Documentation** — [Full Protocol Docs](docs/)
- **Issues** — [Report bugs or request features](../../issues)
- **Discussions** — [Stellar Community Forum](https://community.stellar.org)

---

## License

MIT License — Copyright (c) 2026 KeeperRunNet Protocol.

---

*Automating the future of Soroban, one block at a time.*
