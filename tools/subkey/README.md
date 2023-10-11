# Subkey

## Overview

Subkey is a key management CLI utility for Substrate-based chains relaychain, parachain.

More about this [here](https://github.com/paritytech/polkadot-sdk/blob/master/substrate/bin/utils/subkey).

## Installation

[Source](https://docs.substrate.io/reference/command-line-tools/subkey/)

1. Clone the `polkadot-sdk` repository

   ```sh
   $ git clone https://github.com/paritytech/polkadot-sdk.git
   ```

2. Build the `subkey` binary. As there are so many packages (members) in Cargo.toml's workspace members.

   ```sh
   $ cargo +nightly build --package subkey --release
   ```

3. Install the `subkey` binary by providing the path to the package's `Cargo.toml` file:

   ```sh
   $ cargo install --path substrate/bin/utils/subkey
   ```

## Usage

```sh
$ subkey generate
```
