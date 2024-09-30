<p align="center">
  <a href="https://www.gomaestro.org/">
    <img src="https://www.gomaestro.org/logos/LandingLogos/DarkLogo.svg" alt="Maestro Logo" width="425" />
  </a>
  <h2 align="center">Rust SDK for the <a href="https://www.gomaestro.org/">Maestro</a> Dapp Platform</h2>
  <p align="center">
    <a href="https://docs.gomaestro.org/docs/intro">
      <img src="https://img.shields.io/badge/-Docs-blue?style=flat-square&logo=semantic-scholar&logoColor=white" />
    </a>
    <a href="https://github.com/maestro-org/rust-sdk/blob/main/LICENSE">
      <img src="https://img.shields.io/github/license/maestro-org/rust-sdk?style=flat-square&label=License" />
    </a>
    <a href="https://github.com/maestro-org/haskell-sdk/actions/workflows/build.yml?query=branch%3Amain">
      <img src="https://img.shields.io/github/actions/workflow/status/maestro-org/rust-sdk/main.yml?style=flat-square&branch=main&label=Build" />
    </a>
    <a href="./CONTRIBUTING.md">
      <img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square" />
    </a>
    <a href="https://twitter.com/GoMaestroOrg">
      <img src="https://img.shields.io/badge/-%40GoMaestroOrg-F3F1EF?style=flat-square&logo=twitter&logoColor=1D9BF0" />
    </a>
    <a href="https://discord.gg/ES2rDhBJt3">
      <img src="https://img.shields.io/badge/-Discord-414EEC?style=flat-square&logo=discord&logoColor=white" />
    </a>
    <a href="https://crates.io/crates/maestro-rust-sdk">
      <img src="https://img.shields.io/crates/v/maestro-rust-sdk" />
    </a>
  </p>
</p>

# Getting Started

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Clippy](https://github.com/rust-lang/rust-clippy)
- [Audit](https://docs.rs/cargo-audit/latest/cargo_audit/)

## Usage

```toml
[dependencies]
maestro-rust-sdk = { git = "https://github.com/maestro-org/rust-sdk.git" }
```

```rust
use maestro::Maestro;

let maestro_client = Maestro::new("<PROJECT_API_KEY>", "<NETWORK>")
```

- To generate an API key, create a free account [here](https://dashboard.gomaestro.org/)!
- Network options: `mainnet`, `preprod`, `preview`

## Example

```rust
use maestro::Maestro;

#[tokio::main]
async fn main() {
    let maestro_client = Maestro::new(
        String::from("<PROJECT_API_KEY>"),
        String::from("<NETWORK>"),
    );

    match maestro_client.block_info(9005859).await {
        Ok(block_info) => println!("{}", block_info.data.absolute_slot),
        Err(e) => eprint!("Failed to retrieve block info {}", e),
    }
}
```

# Documentation

- [Maestro public docs](https://docs.gomaestro.org/)

# Contributing

Meastro welcomes all contributors! Please see our [contributing guidelines](CONTRIBUTING.md) and [code of conduct](CODE_OF_CONDUCT.md).
