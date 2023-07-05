//! Arithmetic Calculation in runtime module/pallet
//! 
//! Comming from "Solidity" background, we are used to the intricacies to be taken 
//! care when dealing with arithmetic calculations. For instance, I recommend performing Nr/Dr based calculations
//! especially when there are multiple parameters involved in Nr & Dr.
//! 
//! But in Rust, we don't have to worry about it. Rust takes care of it for us.
//! 
//! Here is an example, where we use `.and_then()` & `.checked_*()` methods to perform arithmetic calculations.
//! NOTE: I have cross-checked the results via testing by throwing a simple use case. 100 * 8/100 = 8.
//! 
//! I also considered the `Nr/Dr` approach, but it gave some errors in terms of overflow in the Dr. side. So, I don't recommend it.
//! 
//! Also, we might have to considering adding trait bounds to take care of the type conversions during arithmetic calculations.
//! The actual data type can be seen during `tests.rs`, but in pallets we would have to deal with traits in order to generalize the pallet
//! with numerous data types possibility.


#[pallet::call]
impl<T: Config> Pallet<T>
// NOTE: Here, `where` clause is required for converting from
// - `BlockNumber` to `BalanceOf<T>`
// - `u128` to `BalanceOf<T>`
// - `u64` to `T::BlockNumber`
// - `u64` to `T::BlockNumber`
// automatically suggested by compiler.
where
    BalanceOf<T>: From<T::BlockNumber> + From<u128> + From<u64>,
    T::BlockNumber: From<u64>,
{

    let interest = principal_amount
        .checked_mul(&interest_rate.into())
        .and_then(|v| v.checked_mul(&maturity_period.into()))
        .and_then(|v| v.checked_div(&fd_epoch.into()))
        .and_then(|v| v.checked_div(&scaling_factor.into()))
        .ok_or("Interest calculation failed")?;
}
