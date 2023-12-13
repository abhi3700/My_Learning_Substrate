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

### Create a random wallet keypair

```sh
$ subkey generate
Secret phrase:       exercise warfare energy arrow poem crawl enlist laundry together resist lock immense
  Network ID:        substrate
  Secret seed:       0xda5afc8039fe870a7e3b4adc91c89e0be667635c0e6a9d847d5ccd53841cf862
  Public key (hex):  0x5a3137ae70a3e4a534ef18b56eb4441ea9000a47cbcfe626d847f9216e21c519
  Account ID:        0x5a3137ae70a3e4a534ef18b56eb4441ea9000a47cbcfe626d847f9216e21c519
  Public key (SS58): 5E6xndC37n8KEbchvRHqXDdrZRp3o87nREFg27wmSNtQhoX1
  SS58 Address:      5E6xndC37n8KEbchvRHqXDdrZRp3o87nREFg27wmSNtQhoX1
```

### Create a wallet keypair from a given seed phrase

```sh
$ subkey inspect "exercise warfare energy arrow poem crawl enlist laundry together resist lock immense"
Secret phrase:       exercise warfare energy arrow poem crawl enlist laundry together resist lock immense
  Network ID:        substrate
  Secret seed:       0xda5afc8039fe870a7e3b4adc91c89e0be667635c0e6a9d847d5ccd53841cf862
  Public key (hex):  0x5a3137ae70a3e4a534ef18b56eb4441ea9000a47cbcfe626d847f9216e21c519
  Account ID:        0x5a3137ae70a3e4a534ef18b56eb4441ea9000a47cbcfe626d847f9216e21c519
  Public key (SS58): 5E6xndC37n8KEbchvRHqXDdrZRp3o87nREFg27wmSNtQhoX1
  SS58 Address:      5E6xndC37n8KEbchvRHqXDdrZRp3o87nREFg27wmSNtQhoX1
```

### Generate node keypair

**Print to console**:

```sh
$ subkey generate-node-key
12D3KooWBUBKF8kV5fNdvbHwLp5DiYoamVFbHY9p2Esxt3tseNh5
af6450bc2acab85e6643a00ab509a1cbb6f8188ea30e3f5f2e5d29c58cc01b56
```

**Save to file**:

```sh
$ subkey generate-node-key --file substrate-node.txt
12D3KooWBUBKF8kV5fNdvbHwLp5DiYoamVFbHY9p2Esxt3tseNh5
```

### Import node keypair

```sh
$ subkey inspect-node-key --file substrate-node.txt
12D3KooWBUBKF8kV5fNdvbHwLp5DiYoamVFbHY9p2Esxt3tseNh5
```
