# Substrate vs EOSIO vs EVM

| Parameter        | Substrate                                                                                              | EOSIO                                | EVM                                            |
| ---------------- | ------------------------------------------------------------------------------------------------------ | ------------------------------------ | ---------------------------------------------- |
| Layer            | L0                                                                                                     | L1                                   | L1                                             |
| SC friendly      | ✅                                                                                                     | ✅                                   | ✅                                             |
| SC binary format | Wasm                                                                                                   | Wasm                                 | EVM bytecode                                   |
| SC language      | ink!                                                                                                   | C++, Rust, Typescript                | Solidity, Vyper, Fe                            |
| SC compiler      | `cargo-contract`                                                                                       | `eosio.cdt`                          | `solc`, truffle, hardhat, brownie, foundry     |
| Node CLI         | `substrate-contracts-node` (more [CLI tools](https://docs.substrate.io/reference/command-line-tools/)) | `cleos`                              | geth, parity, openethereum                     |
| Wallet CLI       | `substrate-contracts-node`                                                                             | `keosd` (also accessed via `cleos`)  | Metamask, Nifty, Trust                         |
| Testnet          | Local, Rococo (relay chain) Canvas (parachain),                                                        | Local, Public (Jungle)               | Local, Public (Goerli)                         |
| Mainnet          | Local, Polkadot, Kusama (canary release)                                                               | Local, Public (EOS, TELOS, WAX, FIO) | Local, Public (Ethereum, Polygon, Fantom, BSC) |
| Assertion        | assert!()                                                                                              | eosio_assert()                       | require()                                      |
