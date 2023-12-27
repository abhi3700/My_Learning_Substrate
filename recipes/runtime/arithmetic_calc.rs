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

use frame_support::sp_runtime::traits::{CheckedDiv, CheckedMul};

#[pallet::call]
impl<T: Config> Pallet<T> {

    // this is how handled in the pallet extrinsics.
    // Here, the types are:
    // - `principal_amount` is `BalanceOf<T>` i.e. Balance
    // - `interest_rate` is `u32`
    // - `maturity_period` is `u32`
    // - `fd_epoch` is `u32`
    // - `scaling_factor` is `u32`
    //
    // - `interest` is `BalanceOf<T>` i.e. Balance
    let interest = principal_amount
        .checked_mul(&Self::u32_to_balance(interest_rate).unwrap())
        .and_then(|v| v.checked_mul(&maturity_period.into()))
        .and_then(|v| v.checked_div(&fd_epoch.into()))
        .and_then(|v| v.checked_div(&scaling_factor.into()))
        .ok_or("Interest calculation failed")?;
}
