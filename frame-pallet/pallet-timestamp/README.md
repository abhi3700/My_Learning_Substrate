# Timestamp Pallet

## Overview

- Timestamp Pallet provides functionality to get and set the on-chain time.
- Repo code: https://github.com/paritytech/substrate/tree/master/frame/timestamp
- They act as inherents which do not have to stand in transaction queue, as they are added by the validator.

## Notes

![](../../img/pallet-timestamp-1.png)

![](../../img/pallet-timestamp-2.png)

They are used to order transactions in the block explorer.

![](../../img/pallet-timestamp-3.png)

![](../../img/pallet-timestamp-4.png)

The Substrate community mutually agreed to 6s block time.

![](../../img/pallet-timestamp-5.png)

When `substrate-node-template` binary is run on CLI, in the "Developer >> Storage" section, we can see the current time as shown below, the time also gets updated as the block gets added to the chain & the time is updated by 6s.

![](../../img/pallet-timestamp-6.png)

In the recent codebase as per `polkadot-v0.9.39`, there are dispatch calls like `block` not available for the timestamp pallet. Hence, just pasting the screenshot. More can be seen in the video referenced down below

## Reference

- [Pallet Timestamp and Inherents | Polkadot Deep Dives](https://www.youtube.com/watch?v=HjtxPcuR8a0)
