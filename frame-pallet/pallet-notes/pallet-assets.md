# Assets Pallet

## Overview

The Assets pallet provides functionality for managing multiple asset. It is sort of like an ERC20 factory which creates ERC20 tokens/assets (fungible).

## Notes

In order to create this pallet, we can use the trait `Currency`, which is a trait that is implemented by the `Balances` pallet. This trait provides the basic functionality for transferring balances (tokens/amounts) between accounts.

Coding-wise, we normally implement the functions like this:

```rust
impl<T: Config> for Pallet<T> {
    // dispatchable functions
}
```

Now, for the `assets` pallet dispatchables creation, we can use the `Currency` trait to implement the functions for the pallet.

💡 This is a way to achieve this.

```rust
impl<T: Config> Currency<T::AccountId> Pallet<T> {
    // dispatchable functions
}
```

---

## References

- [Pallet Assets](https://www.youtube.com/watch?v=eFtWVq6dR0Y)
