# Substrate

## Terminology

- **Relay chain**: The Polkadot super chain which is connected to different parachains.
- **Parachain**: The side-chains which are connected to the main Polkadot relay chain.
- **Collator**: Validator for Polkadot parachain
- **Substrate runtime**: It's like `wasm`, `bytecode` for other blockchain networks.

## ink!

## Comparo with EVM

| EVM Stack                          |
| ---------------------------------- |
| ![EVM stack](../img/evm_stack.png) |

| Substrate Stack                 |
| ------------------------------- |
| ![](../img/substrate_stack.png) |

![](../img/evm_vs_substrate_opcodes.png)

## Why Rust

- Able to write safe code, at compile-time (done using `$ cargo check`), rather than run-time failures in C++
- Fast
- High level compared to C++

> NOTE: Any language can be chosen for Substrate. Parity, Web3 foundation is inclined towards Rust.

## Architecture

![](../img/substrate_arch.png)

Substrate is modular & extensive. Each module is called "Pallet".

## Tutorials

Try out the following tutorials:

- [kitties-tutorial](https://learn.figment.io/tutorials/substrate-kitties-setup), [doc in PDF](tuts/Figment%20Learn%20_%20Substrate%20Kitties%20-%20Basic%20Setup.pdf)

## References

- [Documentation | By Parity](https://docs.substrate.io/main-docs/)
- [Substrate StackExchange](https://substrate.stackexchange.com/)

### Videos

- [Substrate: A Rustic Vision for Polkadot by Gavin Wood at Web3 Summit 2018](https://www.youtube.com/watch?v=0IoUZdDi5Is)
- [Chainlink | Intro to Substrate](https://www.youtube.com/watch?v=o5ANk0sRxEY)
