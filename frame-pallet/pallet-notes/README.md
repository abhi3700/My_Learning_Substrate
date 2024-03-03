# Pallet Notes

## Overview

One should cover in this sequence:

- [x] [Timestamp pallet](./pallet-timestamp.md)
- [x] [Balances pallet](./pallet-balances.md)
- [x] [Democracy pallet](./pallet-democracy.md)
- [ ] [Session pallet](./pallet-session.md)
- [ ] [Treasury pallet](./pallet-treasury.md)
- [ ] [Scheduler Pallet](./pallet-scheduler.md)
- [ ] [Collective pallet](./pallet-collective.md)

> To know more on pallets' usage, please refer to the corresponding test cases.

We need to put all the required FRAME pallets into a runtime. And if we want the runtime to support wasm, we need to create `build.rs` inside `runtime/` folder. <br />
The code is written as:

```rust
use substrate_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::new()
        // Tell the builder to build the project (crate) this `build.rs` is part of.
        .with_current_project()
        // Make sure to export the `heap_base` global, this is required by Substrate
        .export_heap_base()
        // Build the Wasm file so that it imports the memory (need to be provided by at instantiation)
        .import_memory()
        // Build it.
        .build()
}
```

```toml
# as shown in `substrate-node-template` repo.
...
[build-dependencies]
substrate-wasm-builder = { version = "5.0.0-dev", git = "https://github.com/paritytech/substrate.git", optional = true , branch = "polkadot-v1.0.0" }
...
```

Also need to add this in `runtime/src/lib.rs`:

```rust
// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
```

[Source](https://paritytech.github.io/polkadot-sdk/master/substrate_wasm_builder/index.html#prerequisites)
