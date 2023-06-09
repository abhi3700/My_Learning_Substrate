//! Create types that can be used in the storage key-value pair or inside pallet
//!     - dispatchable as local variable type
//!     - storage key-value pair type
//!     - event field type
//!
//! Remember that the trait associated types (AccountId, Balance, etc.) that are being used
//! needs the required traits (Currency, ReservableCurrency, etc.) to be implemented in the
//! pallet via import
//!
//! ```rust
//! use frame_support::traits::{Currency, ReservableCurrency};
//! ```

#[frame_system::pallet]
mod pallet {

    // We can directly use `T::AccountId`. Remember we need to also use
    type _AccountOf<T> = <T as frame_system::Config>::AccountId; // optional
    type BalanceOf<T> =
        <<T as Config>::MyCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
}
