//! Use `Zero::zero()` to get the zero value of a type.
//!
//! Implement this for comparison with a type defined like `BalanceOf<T>` (for instance).
//!
//! Here, `interest` type is defined in the runtime implementation as `Balance` i.e. u128 (mostly).

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
