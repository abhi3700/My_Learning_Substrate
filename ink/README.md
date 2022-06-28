# Ink

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

### 1. Cargo, Rust

- Follow from Solana [here](https://github.com/abhi3700/sol-playground/blob/main/README.md#rustup-rustc-cargo-rustfmt)

### 2.

## References

- [Github repo](https://github.com/paritytech/ink)
- [Documentation](https://ink.substrate.io/)
- Tutorials
  - https://docs.substrate.io/tutorials/smart-contracts/
  - [Substrate contract Web App interactor template - for stock Flipper SC](https://github.com/polk4-net/flipper-app)
  - [By Figment](https://learn.figment.io/protocols/polkadot)
