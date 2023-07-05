//! Use `Zero::zero()` to get the zero value of a type.
//!
//! Implement this for comparison with an associated type defined like `BalanceOf<T>` (for instance).
//!
//! Here, `interest` type is defined in the runtime implementation as `Balance` i.e. u128 (mostly), but in the Balance pallet,
//! it is defined as `BalanceOf<T>` which is an associated type. So, we need to use `Zero::zero()` to get the zero value of `BalanceOf<T>`.
//! And then we can compare it with the `interest` value passed in the `set_fd_interest_rate` function.
//! This is part of a process called "Input Sanitization" which is a good practice to follow.

// cargo add num-traits -p <pallet-name> --no-default-features
use num_traits::Zero;

#[pallet::call_index(0)]
#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
pub fn set_fd_interest_rate(origin: OriginFor<T>, interest: BalanceOf<T>) -> DispatchResult {
    // ====snip====

    // ensure positive interest
    ensure!(interest > Zero::zero(), Error::<T>::ZeroInterestRate);

    // ====snip====

    Ok(())
}
