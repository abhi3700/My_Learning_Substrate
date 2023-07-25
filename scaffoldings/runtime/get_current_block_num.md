---
title: Get Current Block Number
---

```rust
use frame_support::sp_runtime::traits::BlockNumberProvider;

#[frame_support::pallet]
pub mod pallet {
    /// Get the current block number
	impl<T: Config> BlockNumberProvider for Pallet<T> {
		type BlockNumber = T::BlockNumber;

		fn current_block_number() -> Self::BlockNumber {
			<frame_system::Pallet<T>>::block_number()
		}
	}
}
```
