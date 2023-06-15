# Balances Pallet

## Overview

- The balances pallet is designed to be used as the blockchain's underlying currency (where the different traits are implemented). It is extremely optimized for balance movements and transfers, which ensures that the fees that users pay to actually move balances are very low.
- 2 ways to store balance of account:
  - Inside `System` pallet's `AccountData`, precisely inside frame_system's `AccountInfo`.
  - Inside `Balances` pallet under pallet_balances's `Account` storage.
- Abstraction layers of the pallet in form of traits, useful for loose pallet coupling with pallets (your pallet which might require the currency methods). And these abstractions combined with storing the balance on the Frame System level, enables balances to be really "useful" at the runtime level. For example, we **lock** balances for democracy and staking, allowing the same balance to be used twice across two services. On the other hand, we use **reserved** balances for a lot of things like storage deposits making sure that this kind of balance CANNOT be used twice. The traits used by the Balances pallet are:

  - [Currency trait](https://crates.parity.io/frame_support/traits/tokens/currency/trait.Currency.html)
    - has additionally `issue`, `burn` major methods in addition to `Balances` pallet.
  - [ReservableCurrency trait](https://crates.parity.io/frame_support/traits/tokens/currency/trait.ReservableCurrency.html)
  - [NamedReservableCurrency trait](https://crates.parity.io/frame_support/traits/tokens/currency/trait.NamedReservableCurrency.html)
  - [LockableCurrency trait](https://crates.parity.io/frame_support/traits/tokens/currency/trait.LockableCurrency.html)
  - [Imbalance trait](https://crates.parity.io/frame_support/traits/tokens/imbalance/trait.Imbalance.html)

  > It is recommended to use these traits as loose coupling whenever any methods or storage is to be used. I have implemented the same in `vault` pallet in my [`substrate-playground`](https://github.com/abhi3700/substrate-playground).

- `Balance` is the amount of blockchain (relaychain) currency, the type of which is set inside the runtime `runtime/src/lib.rs` file.
- [Dispatchables](https://github.com/abhi3700/substrate-playground)
  - `transfer`
  - `set_balance`
  - `force_transfer`
  - `transfer_keep_alive`
  - `transfer_all`
- This pallet depends on struct [`GenesisConfig`](https://crates.parity.io/pallet_balances/pallet/struct.GenesisConfig.html) defined inside it, to initialize the balances of the treasury accounts like `Alice`, `Alice_stash`, `Bob`, `Bob_stash`, etc. i.e., it can be used to configure the genesis state of this pallet.

![](../../img/pallet-balances-1.png)

- `Balances` vs `Asset` pallet

  - It's relaychain native currency whereas the later is for custom tokens (like ERC20) on
  - It's a single token, whereas the later is multi-token in a single pallet (asset). Hence, the later is kind of like a ERC20 factory.
  - So in essence, you can create your underlying native currency using the balances pallet and if you need additional currencies you can leverage the assets pallet.

- "Imbalance" represents a difference in the token balance (positive or negative) that is yet to be handled or "balanced".

  - Imbalances are used to manage and keep track of changes in the balance of an account as a result of operations like transfers, rewards, and fees. They're part of Substrate's way of abstracting token handling and ensuring safety. Imbalances are created whenever tokens are minted (creating a positive imbalance) or burned/destroyed (creating a negative imbalance).

  - These imbalances must be dealt with and balanced out, usually by the system. For example, when tokens are transferred from one account to another, a negative imbalance is created in the sender's account and a positive imbalance in the receiver's account. The system ensures that these imbalances cancel each other out so that the total supply of tokens remains constant.

  - There are two primary types of imbalances:

    - `PositiveImbalance<T>`: This type of imbalance represents an increase in the total supply of tokens. It's created when tokens are minted or introduced into the system.

    - `NegativeImbalance<T>`: This type of imbalance represents a decrease in the total supply of tokens. It's created when tokens are burned or removed from the system.

  - The `<T>` in `PositiveImbalance<T>` and `NegativeImbalance<T>` is a placeholder for the specific type of token being managed.

  - Imbalances are automatically handled by the Substrate framework. When you're writing a pallet that involves token operations, you usually don't need to deal with imbalances directly, but you do need to be aware of them and understand how they work.

## Notes

### Theory

- Main purposes
  ![](../../img/pallet-balances-2.png)

- **TotalIssuance** vs **Imbalance**
  ![](../../img/pallet-balances-3.png)

  - **TotalIssuance**: This refers to the total number of tokens that have been issued in the blockchain network. It's a dynamic value that _**changes as new tokens are minted (which increases total issuance) or burned (which decreases total issuance)**_. In other words, it's a measure of all tokens currently in existence in the network, including those that might be locked or reserved.

  - **TotalSupply**: This is often used interchangeably with TotalIssuance in many blockchain contexts. It's a measure of the total amount of tokens of all accounts. It changes when tokens moved to/from Imbalance (postive/negative).

  - **Imbalance**: In the Substrate framework, an "Imbalance" represents an unhandled difference in the token balance that's created when tokens are minted or burned. It's a kind of temporary state that's used internally to help manage changes in account balances and ensure that all token operations are safe and consistent. There are two types of imbalances: `PositiveImbalance<T>` (which represents an increase in the total supply of tokens) and `NegativeImbalance<T>` (which represents a decrease in the total supply of tokens). These imbalances are created when tokens are minted or burned, respectively, and they're automatically handled and "balanced out" by the system.

  > Remember that `<T>` in `PositiveImbalance<T>` and `NegativeImbalance<T>` is a placeholder for the specific type of token being managed.

  Here, the Imbalance gets changed when there is any burn or mint operation. For example, if Alice burns 200 units, then the Imbalance will be `NegativeImbalance<T>` of 200 units. Now, this imbalance needs to be balanced out. So, the system will reduce the total supply by 200 units. So, the total supply will be reduced from `1000` to `800` units.

  ![](../../img/pallet-balances-4.png)

  At 2nd level, there is a reduction in the total supply because, the Alice's balance is slashed by 200 units. So, the total supply is reduced by 200 units => from `1000` to `800` units.

  Now, what to do with the Imbalance now? It needs to be balanced out. So, the system will either:

  - transfer to treasury & then use it to incentivize validators/community based on their activity.
  - burn

  > **Observations** in the above example 🔝,
  >
  > - When Alice's balance is reduced by 200 units. Total supply is reduced by 200 units. And the 200 units is moved to Imbalance, hence no change in TotalIssuance.
  >
  > - When the Imbalance was balanced out,
  >   - the 200 units was transferred to treasury. Hence, the TotalIssuance remains unchanged. But the TotalSupply is increased by 200 units as Treasury is an account.
  >   - the 200 units was burned. Hence, the TotalIssuance is reduced by 200 units.

- `Currency` trait & its methods and acts as dependency for other traits.
  ![](../../img/pallet-balances-5.png)
- How `Currency` trait acts as dependency for other traits:
  ![](../../img/pallet-balances-6.png)

---

- **ExistentialDeposit**:
  ![](../../img/pallet-balances-7.png)
- `AccountData`: The Balances pallet revolves around this struct where there are different fields corresponding to which the `free`, `reserved`, etc. balances are maintained.
  ![](../../img/pallet-balances-8.png)

As per the current struct of `AccountData`, this is how the balances are maintained:

![](../../img/pallet-balances-9.png)

The `AccountData` struct in the Substrate `Balances` pallet represents the account balance information. Here's what each field represents:

1. `free`: This is the balance that the account holder has access to and can use freely. This balance is relevant for most operations involving tokens, such as transfers, staking, and paying transaction fees.

2. `reserved`: This balance represents funds that have been locked or reserved for a specific purpose and cannot be used freely by the account holder. Reserved balance could be locked due to reasons like ongoing transactions, staking requirements, or governance votes. It's important to note that the sum of this balance includes all individual holds along with any sums still under the deprecated reserves API.

3. `frozen`: This balance represents a minimum limit that the sum of `free + reserved` cannot fall below. The idea behind the `frozen` balance is to ensure a minimum balance that safeguards against actions where the account owner cannot benefit from a reduction in balance, like slashing (punitive reductions in balance for misbehavior in the network).

4. `flags`: This field holds additional information about the account. The Most Significant Bit (MSB) was used as a flag to indicate if the new reference-counting logic has been implemented for this account. The details of the `ExtraFlags` struct and its use might need to be looked up in the Substrate documentation or codebase for the most current and detailed understanding.

- Existential Deposit (Cont...):
  ![](../../img/pallet-balances-10.png)

---

![](../../img/pallet-balances-11.png)

![](../../img/pallet-balances-12.png)

![](../../img/pallet-balances-13.png)

![](../../img/pallet-balances-14.png)

![](../../img/pallet-balances-15.png)

![](../../img/pallet-balances-16.png)

Here, in the above example 🔝, the reserve_balance keeps getting added whereas the locked_balance ensures the parsed value as min_lock balance. So, when `10` is locked, it gets locked, but then `5` is locked, then no update in locked_balance as it is already `10`. But, when `15` is locked, then the locked_balance gets updated to `15`.

> So, lock_balance is the **max.** of all the locked balances whereas the reserve_balance is the **sum** of all the locked balances.

### Coding

## References

- [Balances Pallet | Polkadot Deep Dives](https://www.youtube.com/watch?v=_FwqB4FwWXk) 🧑🏻‍💻
- [Balances pallet in `crates.parity.io`](https://crates.parity.io/pallet_balances/index.html)
- [substrate stackexchange](https://substrate.stackexchange.com/a/712/2795)
