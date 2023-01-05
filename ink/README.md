# ink!

## About

- Substrate's Contract Pallet (out of all library of pallets) allows Substrate-based chains to run SC on top of it.
- SC language: A eDSL based `rust` language
- SC binary: `wasm` format
- SC compiler: `cargo-contract`
- SC ABI: `metadata`
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

- Follow this [guide](https://github.com/abhi3700/My_Learning-Rust/blob/main/README.md#installation)

### 2. Substrate Framework Pre-requisites

1. `$ rustup toolchain install nightly`
2. `$ rustup target add wasm32-unknown-unknown --toolchain nightly`

### 3. The Substrate SC Node

**Install**/**Update** the substrate node from source code:

`$ cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --force --locked`

![](../img/substrate-contracts-node-install.png)

#### probable issues

- Error: missing protobuf
  - _solution_: `brew install protobuf`

### 4. Install `binaryen` WASM optimizer

- `$ brew install binaryen`

### 5. Linting, Compiler

1. `$ rustup component add rust-src --toolchain nightly-aarch64-apple-darwin` (add rust standard library)
2. `$ cargo install cargo-dylint dylint-link` [lint `ink!` code] i.e. check `ink!` contracts and warn you about issues that might lead to security vulnerabilities.
3. `$ cargo install cargo-contract --force --locked` [**install**/**update** compiler]
   > 1, 2, is for `cargo-contract` to work with `cargo-dylint` & `dylint-link` respectively.

**Verify installation**

```console
❯ cargo-contract --version
cargo-contract 1.5.0-unknown-aarch64-apple-darwin
```

## Getting Started

1. Create a new project

   ```console
   ❯ cargo contract new flipper
   ```

2. Build the project

   ```bash
   ❯ cd flipper
   # dev mode
   ❯ cargo +nightly contract build
   # release mode
   ❯ cargo +nightly contract build --release
   ```

   ```console

   Your contract artifacts are ready. You can find them in:
   ....../flipper/target/ink

     - flipper.contract (code + metadata)
     - flipper.wasm (the contract's code)
     - metadata.json (the contract's metadata)
   ```

   > From debug to release mode, there can be a change in the size of the `wasm` file from `17.5 KB` to `2.6 KB`.

3. Test the project

   ```console
   ❯ cargo test
   ```

4. Deploy the project

   These are the files to be deployed:

   - `flipper.wasm`
   - `metadata.json`

   <!-- TODO: How to deploy? -->

## [SC Standards](./standards.md)

## Smart Contract Development

### Contract

```rust
// Import the ink! module
use ink_lang as ink;

// Define the contract module
#[ink::contract]
mod erc20 {
  // Define the storage struct.

  // Define the events.

  // Define the constructor.

  // Define the public functions.

  // Define the private functions.

  // Define the tests.
}
```

### Struct

```rust
#[ink(storage)]
pub struct Erc20 {
  total_supply: Balance,
  balances: ink_storage::collections::HashMap<AccountId, Balance>,
}
```

### Constructor

Assuming this: **The most basic ERC20 token contract is a fixed supply token**. So is the case here.

A contract can have multiple constructors. The one with no arguments is the `default` one.

```rust
#[ink(constructor)]
pub fn new(initial_supply: Balance) -> Self {
  let caller = Self::env().caller();
  let mut balances = ink_storage::collections::HashMap::new();
  balances.insert(caller, initial_supply);
  Self {
    total_supply: initial_supply,
    balances,
  }
}
```

### Global env

- || to `address` (in Solidity): `AccountId` (formerly it was defined `Address`)
- get the balance of the executed contract: `self.env().balance()` [Source](https://docs.rs/ink_lang/latest/ink_lang/struct.EnvAccess.html#method.balance)
- || to `msg.sender` (in Solidity): `self.env().caller()` or `Self::env().caller()`
- || to `msg.value` (in Solidity):`self.env().transferred_balance()`or`Self::env().transferred_balance()`
- || to `block.timestamp` (in Solidity): `self.env().block_timestamp()` or `Self::env().block_timestamp()`
- || to `block.number` (in Solidity): `self.env().block_number()` or `Self::env().block_number()`
- || to `block.timestamp` (in Solidity): `self.env().block_timestamp()` or `Self::env().block_timestamp()`
- || to `tx.origin` (in Solidity): `self.env().caller_is_origin()` or `Self::env().caller_is_origin()` [Source](https://docs.rs/ink_lang/latest/ink_lang/struct.EnvAccess.html#method.caller_is_origin)
- || get the code hash at a given account id: `self.env().code_hash(&account_id).ok()`
- || to get own code hash: `self.env().own_code_hash().expect("contract should have a code hash")`
- || to check if the contract is called by a contract: `self.env().is_contract(&account_id)`
- returns gas left: `self.env().gas_left()`
- min. balance required for creating an account: `self.env().minimum_balance()`
- || to destroy the contract: `self.env().terminate_contract(&beneficiary)` E.g. `self.env().terminate_contract(&self.env().caller())`
- || to recover the public key of the signer from `signature`, `message_hash`: `self.env().ecdsa_recover(&signature, &message_hash)` [Source](https://docs.rs/ink_lang/latest/ink_lang/struct.EnvAccess.html#method.ecdsa_recover)
- get ETH address from ECDSA compressed public key: `self.env().ecdsa_to_eth_address(&pub_key).expect("must return an Ethereum address for the compressed public key")` [Source](https://docs.rs/ink_lang/latest/ink_lang/struct.EnvAccess.html#method.ecdsa_to_eth_address)
- transfer native coin from SC to an account: `self.env().transfer(&recipient, value).expect("transfer failed")` [Source](https://docs.rs/ink_lang/latest/ink_lang/struct.EnvAccess.html#method.transfer)

Read [more](https://docs.rs/ink_lang/latest/ink_lang/struct.EnvAccess.html)

### Error

We define like as enum:

```rust
mod flipper {
  ...
  ...

  #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
  #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
  pub enum Error {
    /// Returned if non-owner is trying to call.
      OnlyOwnerCanFlip,
  }
  ...
  ...
}
```

### Event

### Data types

- `AccountId`: `u64`
  - e.g. `user: AccountId`
- `Balance`: `u128`
  - e.g. `total_supply: Balance`
- `BlockNumber`: `u32`
  - e.g. `block_num: BlockNumber`
- `ChainExtension`: `::ink_env::NoChainExtension`
  - e.g. `chain_extension: ChainExtension`
- `Hash`: `[u8; 32]`
  - A 32 byte array representing a hash. Here, each element is of `u8` (8-bit) or 1 byte.
  - e.g. `hash: Hash`
- `Mapping`:
  - e.g. `ink_storage::Mapping<AccountId, Balance>`
- `Timestamp`: `u64`
  - e.g. `timestamp: Timestamp`

### Debugging

This is to debug the contract via using `ink_env::debug_println!` function. E.g:

```rust
ink_env::debug_println!("thanks for the funding of {:?} from {:?}", value, caller)
```

## Troubleshooting

### 1. missing `binaryen`: `$ brew install binaryen`

## References

- [ink! Github repo](https://github.com/paritytech/ink)
- [ink! CLI Github repo](https://github.com/paritytech/cargo-contract)
- [ink! Comprehensive Documentation](https://use.ink/)
- [ink! playground](https://playground.substrate.dev/)
- [ink! Rust official doc](https://docs.rs/ink_lang/latest/ink_lang/index.html)
- [CLI tool](https://github.com/paritytech/cargo-contract)
- [Tutorials](https://docs.substrate.io/tutorials/)
  - https://docs.substrate.io/tutorials/smart-contracts/
  - [Substrate contract Web App interactor template - for stock Flipper SC](https://github.com/polk4-net/flipper-app)
  - [By Figment](https://learn.figment.io/protocols/polkadot)
- [Rust before Substrate Ink](https://rust-unofficial.github.io/patterns/intro.html)
- [OpenBrush](https://github.com/Supercolony-net/openbrush-contracts) like OpenZeppelin. It is a library of SCs.
