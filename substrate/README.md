# Substrate

## Installation

Refer [this](../ink/README.md#installation) for installation.

## `substrate-contracts-node` vs `node-template`

Follow [this](../ink!/README.md#3-the-substrate-sc-node)

## Terminology

- **Relay chain**: The Polkadot super chain which is connected to different parachains.
- **Parachain**: The side-chains which are connected to the main Polkadot relay chain. They are more like shards in ETH 2.0.
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

## Blockchain Framework

If someone wants to create a blockchain, then they can use Substrate. It's a framework for creating different blockchain modules. The image below shows the different layers of a blockchain framework:

![](../img/substrate_blockchain_building_block.png)

Here, `Database Layer`, `Networking Layer`, `Consensus Layer` is provided by Substrate.

---

Development wise, here is the diagram which shows the easy to difficult way of using Substrate for your own blockchain:

`Substrate Node` --> `Substrate FRAME` --> `Substrate CORE`

![](../img/substrate_easy_to_core.png)

---

Here is what it looks like before & after creating blockchain using Substrate:

**Before**:
![](../img/substrate_runtime_before_bc.png)

**After**:
![](../img/substrate_runtime_after_bc.png)

## Coding

FRAME is itself a programming language. It's a DSL (Domain Specific Language) for writing Substrate runtime, pallets.

### Pallet

**Start a chain with given/custom runtime**:

- If needs to create a substrate node, then go to `substrate::bin/node/runtime/` & use the codebase. All pallets are used.
- If needs to use a substrate node template, then go to `substrate::bin/node-template/runtime/` & use the codebase. Here, limited pallets are used. You can add more pallets here.
- If needs to create a chain with your own runtime, then just go to `runtime/src/lib.rs` & write your own code.

---

**pallets**: `sudo`, `system`

- `sudo` pallet gives the root permission. It's like `sudo` in Linux. It's more like a on-chain governance.
- `system` pallet is the most important pallet without which no other pallet would work. Infact, it's the core pallet.
- `origin` is the account which is sending the transaction i.e. the caller of the transaction. These are the possible origin values:
  - `None`: For unsigned transactions
  - `Signed`: For signed transactions
  - `Root`: For transactions which are sent by the root
    ![](../img/origin_values.png)

`get` who is making the call here:

```rs
let who = ensure_signed(origin)?;
```

---

**Recovery pallet**

This is mainly to recover your account based on validation given by a set of users (no. defined in the pallet).

### Others

In Substrate, one can create a blockchain which supports any length of account id. Like in Ethereum, it's 20 bytes. In Substrate, it can be any length. That's why we need to use `AccountId` instead of `Address`. And the `AccountId` type can be defined as:

```rs
type AccountId = [u8; 20];  // 20 bytes for Ethereum
type AccountId = [u8; 32];  // 32 bytes for a chain
```

In order to make things generic, we define `Config` like this for defining AccountId, Event, Blocksize, etc.:

```rs
// `frame_system` has already defined the `Config` trait
trait Config {
  type AccountId;
  type BlockNumber;
  // etc...
}
```

And therefore, we get to see `<T>`, `T::`. This is because we are using the `Config` trait.

---

**Gas** in Substrate is called **Weight** (max. value). It's a unit of measurement for the amount of computation required to execute a transaction. It's a measure of the time it takes to execute a transaction.

---

### SC Storage

![](../img/substrate_storage_abstraction_layers.png)

#### A. APIs

The following Storage APIs (data persistence) are available for storage on a substrate blockchain:

- `StorageValue`: Storing a single type in storage.

  - `#[pallet::storage]]`, `#[pallet::getter()]` macros are used for this.
  - can accept any type i.e. `u8`, `String`, etc.
  - `T` is the runtime.

  ![](../img/substrate_storage_value.png)

  - Manipulating `StorageValue`:

  ```rs
  // Put a value in storage
  CountForItems::<T>::put(10);

  // Get the value from storage
  CountForItems::<T>::get();

  // kill a value in storage
  CountForItems::<T>::kill();
  ```

- `StorageMap`: Storing a map from key to value in storage.

  - `#[pallet::storage]]`, `#[pallet::getter()]` macros are used for this.
  - can accept any type as key or val i.e. `u8`, `String`, etc.
  - Here, **[Blake2_128Concat](https://paritytech.github.io/substrate/master/frame_support/struct.Blake2_128Concat.html)** is used as the hashing algorithm. It's a hashing algorithm which is used to hash the key to get the storage key. It's a 128 bit hash.
  - `T::AccountId` is used as the key type.
  - `T` is the runtime.

  ![](../img/substrate_storage_map.png)

  - Manipulating `StorageMap`:

  ```rs
  // Check if a value exists in storage
  let is_false = Items::<T>::contains_key(user);

  // put a value in storage
  Items::<T>::insert(user, new_item);

  // Get the value from storage
  Items::<T>::get(user);

  // kill a value in storage
  Items::<T>::remove(user);
  ```

More complex storage types are also possible.

- `StorageDoubleMap`: Storing a map from 2 keys to single value in storage on a substrate chain.

  ![](../img/substrate_storage_double_map.png)

- `StorageCountMap`: Storing a map from n keys to single value in storage on a substrate chain.

  ![](../img/substrate_storage_n_map.png)

#### B. Overlay Change Set

#### C. Merkle Trie

#### D. KVDB

Key Value Database

### Chain

Here are the steps to create different chains: relay, parachain, parathread, etc.

In order to create a L0 network, we need to create a relay chain. And then, we can create a parachain on top of it. And then, we can create a parathread on top of it. There is a provision of switching b/w parachain & parathread based on their economic viability.

#### Build a local blockchain

[Source](https://docs.substrate.io/tutorials/get-started/build-local-blockchain/)

The `node-template` command-line options specify how you want the running node to operate.

`--dev`:

- chain running in `development` mode & also erases all active data - keys, blockchain database, networking info when <kbd>ctrl+c</kbd> is pressed.
- ensures you have a fresh node every time you run it.

```sh
$ git clone https://github.com/substrate-developer-hub/substrate-node-template
$ cd substrate-node-template
$ git switch -c my-branch-v0.9.29
$ cargo build --release
$ ./target/release/node-template --dev
```

> check for the latest version [here](https://github.com/substrate-developer-hub/substrate-node-template/tags)

![](../img/substrate-node-template.png)

So, here the node is running & producing blocks.

---

View the blocks via `front-end-template`:

```sh
$ git clone https://github.com/substrate-developer-hub/substrate-front-end-template
$ cd substrate-front-end-template
$ gco latest
$ yarn install  // v1.22.1 was not able to download semantic-ui dependency, so chose npm
$ npm i
$ npm run start
```

It opens [this](http://localhost:8000/substrate-front-end-template)

## Tutorials

Try out the following tutorials:

- [kitties-tutorial](https://learn.figment.io/tutorials/substrate-kitties-setup), [doc in PDF](tuts/Figment%20Learn%20_%20Substrate%20Kitties%20-%20Basic%20Setup.pdf)

## Repositories

- [zhubaiyuan/awesome-substrate](https://github.com/zhubaiyuan/awesome-substrate)

## References

- [Documentation | By Parity](https://docs.substrate.io/main-docs/)
  - [Fundamentals](https://docs.substrate.io/fundamentals/)
    - [Transaction & Block Basics](https://docs.substrate.io/fundamentals/transaction-types/)
    - [Runtime Development](https://docs.substrate.io/fundamentals/runtime-development/)
    - [Accounts, Addresses and Keys](https://docs.substrate.io/fundamentals/accounts-addresses-keys/)
    - [Polkadot JS Client](https://docs.substrate.io/fundamentals/light-clients-in-substrate-connect/)
    - [Rust for Substrate](https://docs.substrate.io/fundamentals/rust-basics/)
  - [Tutorials](https://docs.substrate.io/tutorials/)
    - [SC](https://docs.substrate.io/tutorials/smart-contracts/)
      - [Build a Token SC](https://docs.substrate.io/tutorials/smart-contracts/build-a-token-contract/)
  - [Reference](https://docs.substrate.io/reference/)
    - [FRAME macros](https://docs.substrate.io/reference/frame-macros/)
    - [Cryptography](https://docs.substrate.io/reference/cryptography/)
- [Substrate StackExchange](https://substrate.stackexchange.com/)
- [Substrate Recipes](https://substrate.recipes/introduction.html)
- [Substrate Rust doc](https://paritytech.github.io/substrate/)

### Videos

- [Substrate: A Rustic Vision for Polkadot by Gavin Wood at Web3 Summit 2018](https://www.youtube.com/watch?v=0IoUZdDi5Is)
- [Chainlink | Intro to Substrate](https://www.youtube.com/watch?v=o5ANk0sRxEY)
- [Sub0 Online: Storage on a Substrate chain](https://www.youtube.com/watch?v=ek8SzCCk59o)
- [Sub0.1: Storage on Substrate - Shawn Tabrizi, Developer experience at Parity](https://www.youtube.com/watch?v=kKKOL20FdII)
