# ink!

## About

- Substrate's Contract Pallet (out of all library of pallets) allows Substrate-based chains to run SC on top of it.
- SC language: A eDSL based `rust` language
- SC binary: `wasm` format
- Testnet (local): Kickstart your own substrate parachain with [substrate-contracts-node](https://github.com/paritytech/substrate-contracts-node) & to view on block explorer:
- Testnet (public):
  - Rococo (Relay chain)
    - Canvas (parachain) supporting `ink!` SC. It is substrate-based.
- Mainnet (Relay chain)
  - Polkadot
  - Kusama (for canary release): The newer features are first integrated here & then launched on Polkadot as stable version like LTS vs Latest in NodeJS.
    > Unlike other blockchains, where there are 1 mainnet, here there are 2 relay chains. They did this to ensure test happening on Kusama with real tokens, not with the faucet ones.

## Installation

> Below guide is for macOS (M1)

### 1. Cargo, Rust

- Follow from Solana [here](https://github.com/abhi3700/sol-playground/blob/main/README.md#rustup-rustc-cargo-rustfmt)

### 2. CLI tool: `cargo-contract`

```
❯ cargo install cargo-contract --force
```

> NOTE: You might get prompt to install `dylint-link` before this. Do this using `❯ cargo install dylint-link`

Verify installation

```console
❯ cargo-contract -V                                                                                                                         ⏎
cargo-contract 1.4.0-unknown-x86_64-apple-darwin
```

## [Standards](./standards.md)

## References

- [ink! Github repo](https://github.com/paritytech/ink)
- [ink! CLI repo](https://github.com/paritytech/cargo-contract)
- [ink! Documentation](https://ink.substrate.io/)
- [CLI tool](https://github.com/paritytech/cargo-contract)
- Tutorials
  - https://docs.substrate.io/tutorials/smart-contracts/
  - [Substrate contract Web App interactor template - for stock Flipper SC](https://github.com/polk4-net/flipper-app)
  - [By Figment](https://learn.figment.io/protocols/polkadot)
- [Rust before Substrate Ink](https://rust-unofficial.github.io/patterns/intro.html)
- [OpenBrush](https://github.com/Supercolony-net/openbrush-contracts) like OpenZeppelin. It is a library of SCs.
