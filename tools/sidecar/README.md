# Sidecar

## Overview

REST API to connect to substrate based chains like Polkadot, Kusama to get all info related to node, chain, storage, pallet, events, etc.

The default ports for Polkadot RPC are **9933** for HTTP and **9944** for WS.

You can also spin up your own node using `./target/release/node-template` and then feed the http, wss url to the sidecar.

## Usage

### Using curl

Just use this swagger page for getting info about Polkadot, Kusama chains. There might be more API endpoints required. Please raise an issue if you find any.

```sh
$ curl -X 'GET' \
  'https://polkadot-public-sidecar.parity-chains.parity.io/blocks?range=0-10&eventDocs=false&extrinsicDocs=false' \
  -H 'accept: application/json'
```

> We can make our own rust client for this API.

## References

- [substrate-sidecar-api](https://paritytech.github.io/substrate-api-sidecar/dist/) RPC API for substrate based chains.
- [API Sidecar and TX Wrapper | Polkadot Deep Dive](https://www.youtube.com/watch?v=8oqZhdl7C1o)
