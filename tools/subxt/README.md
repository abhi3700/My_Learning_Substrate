# subxt

## Overview

It's a rust library that interacts with substrate based chains/nodes. It's capable of reading:

- blocks
- constants
- event
- extrinsic
- storage
- subscribing blocks

## Installation

### CLI

Install `subxt-cli` using cargo:

```sh
$ cargo install subxt-cli
```

#### Usage

In order to download the metadata of a chain, you can use the following command:

```sh
$ subxt-cli --url wss://rpc.polkadot.io > hello.metadata.scale
```

> You can also get your substrate node's metadata using `--url wss://`

## References

- [subxt book](https://docs.rs/subxt/latest/subxt/book/index.html)
