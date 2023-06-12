# Timestamp Pallet

## Overview

- Timestamp Pallet provides functionality to get and set the on-chain time.
- Repo code: https://github.com/paritytech/substrate/tree/master/frame/timestamp
- They act as inherents which do not have to stand in transaction queue, as they are added by the validator.

## Notes

### Theory

![](../../img/pallet-timestamp-1.png)

![](../../img/pallet-timestamp-2.png)

They are used to order transactions in the block explorer.

![](../../img/pallet-timestamp-3.png)

![](../../img/pallet-timestamp-4.png)

The Substrate community mutually agreed to 6s block time.

![](../../img/pallet-timestamp-5.png)

When `substrate-node-template` binary is run on CLI, in the "Developer >> Storage" section, we can see the current time as shown below, the time also gets updated as the block gets added to the chain & the time is updated by 6s.

![](../../img/pallet-timestamp-6.png)

In the recent codebase as per `polkadot-v0.9.39`, there are dispatch calls like `block` not available for the timestamp pallet. Hence, just pasting the screenshot. More can be seen in the video referenced down below.

### Coding

- Structure of the pallet.

  ```
  [Timestamp mod]
      |__ [Config]
      |__ [Pallet struct]
      |__ [Storage]
      |__ [Hooks]
      |__ [Impl Pallet with extrinsics]
      |__ [Impl Pallet with intrinsics]
  [Impl Pallet]
  [Impl Time for Pallet]
  [Impl UnixTime for Pallet]
  ```

- Here, we are using 2 uncommon components - `Hooks`, `Inherent`.
- The call `set` makes an inherent call.
  ![](../../img/pallet-timestamp-8.png)
- The `block` dispatchable is not available in the recent codebase as per `polkadot-v0.9.39`. Hence, just pasting the screenshot. More can be seen in the video [here](https://youtu.be/HjtxPcuR8a0?list=PLOyWqupZ-WGsfnlpkk0KWX3uS4yg6ZztG&t=656)
  ![](../../img/pallet-timestamp-9.png)
- The corresponding `code` for `block` extrinsic is as follows:
  ![](../../img/pallet-timestamp-10.png)
- If this `block` extrinsic was run, then the chain stopped. Here is the demo [video](https://youtu.be/HjtxPcuR8a0?list=PLOyWqupZ-WGsfnlpkk0KWX3uS4yg6ZztG&t=679) & screenshot:
  ![](../../img/pallet-timestamp-11.png)

  The time keeps on running to 14s, but there are no blocks. Hence, the chain panics with a msg - "Timestamp must be updated once in the block" as shown in CLI:
  ![](../../img/pallet-timestamp-12.png)

  as per the `on_finalize` function shown below. Basically, the error is thrown when trying to finalize the last block that was authored.

- The timestamp whether set or not is checked during `finalization` using this code snippet inside `Hooks` trait.
  ![](../../img/pallet-timestamp-7.png)
- We can also unblock this by `block` with value as `No`.
  ![](../../img/pallet-timestamp-13.png)

  And then the block starts ticking time.

- Add a custom inherent data (remark)
  old code of `pallet-timestamp`:
  ![](../../img/pallet-timestamp-14.png)

  As of today [(June-2023)](https://github.com/paritytech/substrate/releases/tag/monthly-2023-06), there is only 1 extrinsic - [`store`](https://github.com/paritytech/substrate/blob/85415fb3a452dba12ff564e6b093048eed4c5aad/frame/remark/src/lib.rs#L67-L78) in `remark` pallet, which is used to store offchain data. So, with the above code, you can find in the referenced video that the remark once set with a bytes value, can be seen in every block after timestamp set.

- [assert!](https://github.com/paritytech/substrate/blob/85415fb3a452dba12ff564e6b093048eed4c5aad/frame/timestamp/src/lib.rs#L206-L210) here checks for the current time i.e. `now` (function param) to be either zero (in case of genesis block) or greater than the last block time by min. `6s`. This is to ensure that the time is always increasing.
- `create_inherent` is run by the block author.
- `check_inherent` is run by every other node except the block author.
  > In this way, we can check for 2/3 majority of nodes to be in sync with the time. Hence, the consensus is achieved. This is how time synchronization in the distributed system is achieved.
- In `check_inherent`, there are 2 params:
  - `call_data`: the data that is passed by the block author. This data comes from the block imported by the non-author node.
  - `inherent_data` - the data that is passed by this node (except block author).
  - The max. drift allowed is 30s. Hence, the block imported by non-author node if has set time more than 30s than the non-author's latest block timestamp, then the block is rejected.
  - If the `received_t` is less than the min. allowed time i.e. 6s, then the block is rejected.
  - So, the `inherent_data`'s time & `call_data`'s time should be within 30s i.e. <= 30s. And the `call_data`'s time should be >= 6s. Then, the block is accepted.

## Reference

- [Pallet Timestamp and Inherents | Polkadot Deep Dives](https://www.youtube.com/watch?v=HjtxPcuR8a0) 🧑🏻‍💻
