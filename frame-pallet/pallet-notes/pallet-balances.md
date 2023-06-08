# Balances Pallet

## Overview

- The balances pallet is designed to be used as the blockchain's underlying currency. It is extremely optimized for balance movements and transfers, which ensures that the fees that users pay to actually move balances are very low.
- 2 ways to store balance of account:
  - Inside `System` pallet's `AccountData`, precisely inside frame_system's `AccountInfo`.
  - Inside `Balances` pallet under pallet_balances's `Account` storage.
- Abstraction layers of the pallet in form of traits, useful for loose pallet coupling with pallets (your pallet which might require the currency methods):

  - [Currency trait](https://crates.parity.io/frame_support/traits/tokens/currency/trait.Currency.html)
    - has additionally `issue`, `burn` major methods in addition to `Balances` pallet.
  - [ReservableCurrency trait](https://crates.parity.io/frame_support/traits/tokens/currency/trait.ReservableCurrency.html)
  - [NamedReservableCurrency trait](https://crates.parity.io/frame_support/traits/tokens/currency/trait.NamedReservableCurrency.html)
  - [LockableCurrency trait](https://crates.parity.io/frame_support/traits/tokens/currency/trait.LockableCurrency.html)
  - [Imbalance trait](https://crates.parity.io/frame_support/traits/tokens/imbalance/trait.Imbalance.html)

  > It is recommended to use these traits as loose coupling whenever any methods or storage is to be used. I have implemented the same in `bank` pallet in my [`substrate-playground`](https://github.com/abhi3700/substrate-playground).

- `Balance` is the amount of blockchain (relaychain) currency, the type of which is set inside the runtime `runtime/src/lib.rs` file.
- [Dispatchables](https://github.com/abhi3700/substrate-playground)
  - `transfer`
  - `set_balance`
  - `force_transfer`
  - `transfer_keep_alive`
  - `transfer_all`
- This pallet depends on struct [`GenesisConfig`](https://crates.parity.io/pallet_balances/pallet/struct.GenesisConfig.html) defined inside it, to initialize the balances of the treasury accounts like `Alice`, `Alice_stash`, `Bob`, `Bob_stash`, etc. i.e., it can be used to configure the genesis state of this pallet.

![](../../img/pallet-balances-1.png)

## References

- [Balances Pallet | Polkadot Deep Dives](https://www.youtube.com/watch?v=_FwqB4FwWXk)
- [Balances pallet in `crates.parity.io`](https://crates.parity.io/pallet_balances/index.html)
