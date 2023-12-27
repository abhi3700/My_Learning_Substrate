//! Most common traits & types, and their imports
//! Required to use their methods in the pallet's dispatchables.
//!
//! You need to provide their implementation in runtime like this:
//!
//! ```rust
//! impl pallet_template::Config for Runtime {
//!     type MyCurrency = Balances;
//!     type MyBalance = u32;
//! }
//! ```
//!
//! If required, we might have to add crate into `Cargo.toml` as well
//! ```bash
//! // mostly without default
//! $ cargo add <crate> --no-default-features
//! ```

// ###############################################################
// pallets/<pallet-name>/src/lib.rs
// ###############################################################

#[frame_support::pallet]
pub mod pallet {
    // imports
    use frame_support::traits::{Currency, LockableCurrency, ReservableCurrency};

    //==========TYPES================
    // NOTE: Types are created after Config trait's associated types
    type AccountOf<T> = <T as frame_system::Config>::AccountId;

    // balance type used for any amount of currency say....
    // Usage: `fn stake(origin, amount: BalanceOf<T>)`
    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// hash type used for any hash
    /// Usage: `fn verify(origin, hash: HashOf<T>)`
    type HashOf<T> = <T as frame_system::Config>::Hash;

    //==========ASSOCIATED TYPES of trait================
    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        // Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        // MyCurrency type for this pallet. Here, we could have used `Currency` trait.
        // But, we need to use `reserved_balance` function which is not available in `Currency` trait.
        // That's why `ReservableCurrency`, `LockableCurrency` are used.
        type MyCurrency: ReservableCurrency<Self::AccountId> + LockableCurrency<Self::AccountId>;
    }
}

// ###############################################################
//========impl block in RUNTIME | runtime/src/lib.rs==============
// ###############################################################
// NOTE: the types for which we don't provide the implementation like RuntimeEvent, Hash, Balance are already
// implemented via `frame_system::Config`, `pallet_balances`
impl pallet_template::config for Runtime {
    type RuntimeEvent = System;
    type MyCurrency = Balances;
}
