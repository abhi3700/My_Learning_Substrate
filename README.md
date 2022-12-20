# My_Learning_Polkadot

Learn everything about Polkadot, Kusama, [Substrate](./substrate/), etc.

## About

- Layer-0 i.e. L0
- chains (mainnet or testnet):

  - Relay chain
    - main/central chain
    - validators (297 approx.) ensuring the security of the network i.e. Polkadot
  - Parachains (100 max.)
    - collators (5 approx.)
    - they need to stake some auction fee in DOT (held every 6 months) to register a parachain on the relay chain. So, it's a weighted-fee.
  - slots
    - parachain slot (limited)
    - parachain bridge slot
    - parathread slot

- Token details:
  - symbol: DOT
  - decimal: 10 (now, earlier 12 before 21-aug-2020)
- Consensus Algorithm (NPoS):

  - **voter/nominator**:
    - Anyone who has min. `10` DOT on Polkadot (0.1 KSM on Kusama).
    - Max. `50,000` nominators allowed for electing the validators.
    - Max. `16` validators on Polkadot (`24` on Kusama) can be nominated by a nominator.
    - The staked amount `100 DOT` (say) is divided among the selected validators (say `16` or `6`) equally i.e. `6.25 DOT` each.
    - A voter can nominate less than `16` validators as well. But, the chances of validators getting selected are less. Also, the rewards are less.
    - Only the top `256` nominators for a specific validator get paid.
    - **Cons**:
      - One might loose their DOT in case of electing a malicious validator.
      - One might not gain any rewards
        - if none of the selected validator is elected.
        - if very few validators are elected. So, it's recommended to nominate max. `16` validators. Hence, one needs to do proper research.
  - **validator**:
    - Anyone who has min. 'x' (dynamic, changes over time) DOT on Polkadot to become active validator.
    - They need to stake their DOTs to get elected as a validator.
    - They can lose their staked DOTs, if they turn out to be malicious.
    - One has to verify their decentralized identity on Polkadot. Hence, would get ✅ mark on their profile. The ones whose identity is under verification would get ☑️ (grey color) mark.
    - One can be very popular if they are oversubscribed by the nominators.

  [Best Practices for Nominating](https://support.polkadot.network/support/solutions/articles/65000150130-how-do-i-know-which-validators-to-choose-)

  [How do I know which validators to choose?](https://support.polkadot.network/support/solutions/articles/65000150130-how-do-i-know-which-validators-to-choose-)

- **Mainnet**:
  - relaychains:
    - Kusama (new feature launched here 1st),
    - Polkadot (at stable stage after feature launched on Kusama)
- **Testnet**:
  - relaychain: Rococo
    - faucet: Rococo Faucet
    - parachain: Canvas, Contracts (by Parity)
- There are 2 ways to develop dApp on a substrate chain:

  - M-1: write SC on native parachain/parathread using `ink!` (ink! is a Rust-based eDSL for writing Wasm smart contracts)
  - M-2: write SC on EVM supported parachain/parathread using `solidity` (solidity is a high-level language for implementing smart contracts)

- pay fee with DOT for 2 different types of chains:

  - Parachain: weighted fee.
  - Parathread: participate on per use i.e. block basis or essentially `pay-as-you-go`.

- **Parachain vs Parathread**:

| Parachain                             | Parathread                                                    |
| ------------------------------------- | ------------------------------------------------------------- |
| 1. Parachain is always connected.     | 1. Parathread is not always connected.                        |
| 2. DOT staked, hence weight-fee model | 2. pay per block basis or pay-as-you-go or gas-metering model |
| 3. supports SC                        | 3. supports SC                                                |

- If you think about **Polkadot** as a giant computer, **parachains** are like applications that are in physical memory and highly available. **Parathreads** are like applications that are on disk and can be copied into memory when needed. For those already familiar with how **Bitcoin** and **Ethereum** work, users bid to enter a **parathread** block into the relay chain similar to how users bid to include a transaction in a **Bitcoin** or **Ethereum** block.

- Parathreads are ideal for three types of applications:

  - applications seeking an on-ramp to Polkadot,
  - applications worried about losing parachain slots, and
  - applications that have more reads than writes.

- Parathreads increases the number of applications that can operate on Polkadot by pooling them to share parachain slots. This will allow more infrastructure chains and improve composability."

![](img/parachain_vs_parathread.png)

> Parachains require `20,000 DOT` tokens to be staked in order to secure a slot. And these projects don't have enough budget can use Parathreads so that they can come back to the parachain slot auction later.

```mermaid
graph LR
Parachain --when less budget--> ParaThread --when more budget--> Parachain
```

- **Polkadot vs SC**:
  "Polkadot does not use a gas-metering model, but a weight-fee model, meaning Parachains do not have a gas-metered model in their logic. Parachains can implement powerful programming languages. Better put, parachains try to be proactive, while smart contract platforms are event-driven."

- **EVM vs WASM**:

| EVM                                                                      | WASM                                                                                                                        |
| ------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------- |
| it isn’t as fast as it should be and directly impact network efficiency  | it expands the supported languages that a developer can write a smart contract in, such as Rust, C/C++, C#, and Typescript. |
| it uses opcodes that are not optimized for different hardware platforms. | it is is highly performant.                                                                                                 |
| The support and reach for developing smart contracts are limited.        | it is optimized for different hardware platforms.                                                                           |

- **Pallet**: module that trigger functionality on a parachain. Typically like system contracts on a L1 Blockchain.

## Installation

## Quickstart

- Follow [this](https://docs.substrate.io/quick-start/)

## Parachains

### Acala

Hub for DeFi in Polkadot, Kusama.

- Acala is on **Polkadot**
- Karura is on **Kusama**

### Moonbeam

EVM ✅

- [Developer docs](https://docs.moonbeam.network/builders/)
- [Collators](https://moonbeam.moonscan.io/collators)
- [Block explorers](https://docs.moonbeam.network/builders/get-started/explorers/)
- [Wallets](https://docs.moonbeam.network/builders/integrations/wallets/)
- [Pallets](https://docs.moonbeam.network/builders/pallets-precompiles/pallets/)

## Block Explorer

Switch to any L0, parachain, parathread network.

- [Subscan](https://polkadot.subscan.io/)
- [Polkadot.js](https://polkadot.js.org/apps/#/explorer)
- [Polkascan](https://explorer.polkascan.io/)

## Wallet

These are the wallets (like Metamask):

URL: https://wiki.polkadot.network/docs/build-wallets

---

Browser extension: https://chrome.google.com/webstore/detail/polkadot%7Bjs%7D-extension/mopnmbcafieddcagagdcbnhejhlodfdd/related

---

Mobile wallets: https://polkawallet.io/

## Development

- Substrate allows the different features of Polkadot as it allows a modular design with pallets. Using different pallets like balance, account, transaction, block, consensus one can create own parachain.
- Substrate (by default) has a `WASM` runtime but it also allows to create a custom runtime like `EVM` using pallet programming. So, if someone wants to create a `EVM` runtime for running EVM SCs, then a set of pallet (account, balance, consensus, etc.) has to be created in order to join them together & form the required runtime for a parachain with that runtime.
- `FRAME` is the library which is used to build DApps.

![](img/substrate_structure.png)

There are different FRAME pallets to choose from in order to add into runtime.

![](img/substrate_frame.png)

![](img/anatomy_of_substrate_node.png)

For networking, substrate use `libp2p` protocol (developed by IPFS creators).

![](img/polkadot_modules.png)

![](img/substrate_node_template.png)

Using this template, we can start a chain locally. [repo](https://github.com/substrate-developer-hub/substrate-node-template).

Using the front-end template, we can view the information on a website. [repo](https://github.com/substrate-developer-hub/substrate-front-end-template).

Substrate pallets:
![](img/substrate_pallet.png)

The development workflow to pallet:
![](img/workflow_to_pallet.png)

where, `sp: substrate primitive`, `sc: substrate client`

## Auction

In Polkadot, in order to secure a slot, a project/parachain has to bid in the auction. This auction has no end time. The winning is completely based on the highest amounts in terms of ranking.

This auction is similar to Candle auction in 16th century.

![](img/polkadot_auction.png)

There is also a feature of crowdloan where anyone holding DOT can lend to project/parachain. And after the auction period is over, the DOTs are returned back to the owners permissionlessly.

## Tools

![](img/built_with_substrate.png)

- [Substrate](./substrate/)
  - [playground](https://playground.substrate.dev/): setup w/o local PC.
- [Telemetry](https://telemetry.polkadot.io/): View all the nodes running polkadot.
- Polkadot explorer
  - polkascan: https://explorer.polkascan.io/polkadot
  - https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpolkadot.api.onfinality.io%2Fpublic-ws#/explorer
- kusama explorer
  - polkascan: https://explorer.polkascan.io/kusama
  - https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama.api.onfinality.io%2Fpublic-ws#/explorer

## Packages

All web3 packages here:

- [`polkadot-js` | JS](https://polkadot.js.org/docs/)
- [Python Substrate Interface | Python](https://github.com/polkascan/py-substrate-interface)
- [REST service for substrate blockchains | TS](https://github.com/paritytech/substrate-api-sidecar)

## Polkadot vs Kusama

- All the new features are deployed on Kusama 1st & then it gets deployed on Polkadot.
- There are plans to brige both these relay chains

## Interoperability

There are message protocols which are used for communicating b/w chains. E.g. XCM, XCMP

- XCM: Cross Consensus Message format
- XCMP: Cross Chain Messaging Protocol

![](img/interoperability.png)

## Network architecture

![](img/network_architecture.png)

![](img/legends.png)

The **collators** give the **proof** (parachains' block when reached finality i.e. 'proof of validity') to the **validators** who then add to the **relay chain**.

Each parachain has 2 options of adding blocks into the relay chain:

- slot (`stake DOT` & secure position)
- parathread pool (`pay-as-you-go` model)

## State transition

![](img/parachain_state.png)

It stays in merkle tree.

![](img/parachain_state_transition.png)

Here, only the nodes which have been changed in terms of data, those gets added in hash. The figure above shows the transition state.

## Proof of Validity

![](img/proof_of_validity.png)

## Validation process

A receipt is used to put into the blocks of relay chain

![](img/receipt.png)

## Upgrades

![](img/forkless_upgrades.png)

There is a `set_code` function which allows to upgrade the network.

![](img/forkless_upgrades_2.png.png)

It shows comparo b/w legacy & polkadot blockchains.

## References

- [Parity Homepage](https://www.parity.io/)
- [Substrate Homepage](https://substrate.io/)
- [Parity Tech Rust Doc](https://paritytech.github.io/)
  - [Substrate Rust doc](https://paritytech.github.io/substrate/)
  - [Polkadot Rust doc](https://paritytech.github.io/polkadot/)
  - [Cumulus Rust doc](https://paritytech.github.io/cumulus)
  - [Wasmi Rust doc](https://paritytech.github.io/wasmi)

### Blogs

- [Polkadot Blockchain Terms](https://learn.figment.io/tutorials/polkadot-blockchain-terms)
- [HOW TO BUILD PARACHAIN ON POLKADOT](https://www.leewayhertz.com/build-parachain-on-polkadot/)
- [Polkadot Messaging Scheme (XCMP)](https://deeprnd.medium.com/polkadot-messaging-scheme-xcmp-afcdb9b52616)
- [Support: Ask any Q.s here](https://support.polkadot.network/support/home)

### Videos

- [Gavin Wood: Explaining the Polkadot Launch Process](https://www.youtube.com/watch?v=TpcCeo-ZkDY)
- [Chainlink | Introduction to Polkadot, Parachains, and Substrate](https://www.youtube.com/watch?v=gT-9r1bcVHY) ✅
- [The Path of a Parachain Block on Polkadot and Kusama Network](https://www.youtube.com/watch?v=m0vxqWwFfDs)
- [Intro to Substrate codebase and FRAME pallet deep-dive with Joe Petrowski and Shawn Tabrizi](https://www.youtube.com/watch?v=5PSllaWbYag) ✅
- [Learn Rust syntax for Substrate | Substrate Seminar](https://www.youtube.com/watch?v=VVU3Io2dACY) ✅
- [Polkadot and Substrate – Build Custom Blockchains that can Evolve](https://www.youtube.com/watch?v=HrTSpIqhyc0)
- [Sub0 Online: ink! 3.0: A Rust & Smart Contracts Love Story](https://www.youtube.com/watch?v=WNNJzK3dGGQ)
