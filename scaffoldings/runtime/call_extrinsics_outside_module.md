---
title: Call Extrinsic Outside Module
---

**How to call extrinsics outside of the module inside the pallet `lib.rs` file?**

Using `Call::<extrinsic-name> {arg1, arg2}` like this:

> You can exclude `origin` param.

```rust
pub use pallet::*;  // MUST use this line

#[frame_system::pallet]
pub mod pallet {
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        fn submit_price(origin, price) -> DispatchResult {
            // snip ...
            Ok(())
        }
    }
}

// define utils outside the pallet module
impl<T> Pallet<T> {
    fn validate_unsigned() -> TransactionValidity {
        // snip ...

        Call::submit_price { price };        // NOTE: Observe this line
    }
}
```
