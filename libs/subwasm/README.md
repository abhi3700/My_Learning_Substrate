# Subwasm

## Installation

### Using Cargo

```sh
cargo install --locked --git https://github.com/chevdor/subwasm --tag v0.16.1
```

### Using Homebrew for macOS

```sh
brew tap chevdor/subwasm https://github.com/chevdor/subwasm
brew install subwasm
```

## Usage

### `get`

Get/Download the runtime wasm from a running node through rpc

```console
❯ subwasm get
❯ subwasm get http://localhost:9933
❯ subwasm get http://localhost:9933 --output runtime_000.wasm
```

### `info`

returns summarized information about a runtime

```console
❯ subwasm info
🏋️  Runtime size:                0.213 MB (223,474 bytes)
🗜  Compressed:                  Yes, 78.46%
✨ Reserved meta:               OK - [6D, 65, 74, 61]
🎁 Metadata version:            V14
🔥 Core version:                node-template-100 (node-template-1.tx1.au1)
🗳️  system.setCode hash:         0x940457ac0661319310f220895d294444be4e1d882ebcdddfbe0f645463641008
🗳️  authorizeUpgrade hash:       0xb65257be9f09ba2b3f943dfe5eaf55b1594a45523f933946ef1bcb3b17b858bf
#️⃣  Blake2-256 hash:             0xac34e7a2af5053123f911e7bf6eea12224c54885b1c4bc308ea359ed74a1d29a
📦 IPFS:                        https://www.ipfs.io/ipfs/Qmaf94VgwUfce66sUjbPbPZAa2zvaW4mhiBnw1iUrrmeFE
```

---

Passing wasm file

```console
❯ subwasm info runtime_000.wasm
🏋️  Runtime size:                0.213 MB (223,474 bytes)
🗜  Compressed:                  Yes, 78.46%
✨ Reserved meta:               OK - [6D, 65, 74, 61]
🎁 Metadata version:            V14
🔥 Core version:                node-template-100 (node-template-1.tx1.au1)
🗳️  system.setCode hash:         0x940457ac0661319310f220895d294444be4e1d882ebcdddfbe0f645463641008
🗳️  authorizeUpgrade hash:       0xb65257be9f09ba2b3f943dfe5eaf55b1594a45523f933946ef1bcb3b17b858bf
#️⃣  Blake2-256 hash:             0xac34e7a2af5053123f911e7bf6eea12224c54885b1c4bc308ea359ed74a1d29a
📦 IPFS:                        https://www.ipfs.io/ipfs/Qmaf94VgwUfce66sUjbPbPZAa2zvaW4mhiBnw1iUrrmeFE
```

### Diff

Compare 2 runtimes

```console
❯ subwasm diff runtime_000.wasm runtime_001.wasm
Running subwasm v0.16.1
  🅰️  File("runtime_000.wasm")
  🅱️  File("runtime_001.wasm")
  ✅  Both size are identical: 0.213 MB (223,474 bytes)
Checking metadata versions:
  ✅ Both metadata versions are identical: V14
Checking core versions:
  ✅  The 2 core versions are identical: node-template-100 (node-template-1.tx1.au1)
Checking runtime metadata:
  ✅  The metadata are identical
Comparing V14 with V14
Changes:
- unmodified    = 100.00% (1)
- added         =  0.00% (0)
- modified      =  0.00% (0)
- removed       =  0.00% (0)
Skipped:
- documentation =     0
- bytes         =     0
```

## Resources

- [Subwasm on Github](https://github.com/chevdor/subwasm)
