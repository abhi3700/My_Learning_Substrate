//! Define `Currency` (like ERC20 token in SC world) type as `ReservableCurrency` type
//! 0. Import the `ReservableCurrency` & `Currency` traits from `frame_support::traits`
//! 1. Define the type inside `Config` trait of the pallet
//! 2. Define types - `AccountOf<T>` & `BalanceOf<T>` outside the `Config` trait
//! 3. In `runtime/src/lib.rs` file, inside the `impl` block, set the type `Currency` as `Balances`
//! 
//! Source (answered by Shawn): https://substrate.stackexchange.com/questions/954/tbalance-associated-type-balance-not-found-for-t

// ===== inside pallet `src/lib.rs` file

/// 2. Define types - `AccountOf<T>` & `BalanceOf<T>` outside the `Config` trait
type AccountOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// 1. Define the type inside `Config` trait of the pallet
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + Into<<T as frame_system::Config>::RuntimeEvent>
    type Currency: ReservableCurrency<Self::AccountId>
}

/// Inside a pallet, you can compare the reserve_balance with the minimum balance of an account like this:
let minimum_balance = T::Currency::minimum_balance();
let current_reserve_balance = T::Currency::reserve_balance(&caller);
ensure!(current_reserve_balance >= minimum_balance, Error::<T>::InsufficientReserves);

// ===== inside runtime `src/lib.rs` file
/// 3. In `runtime/src/lib.rs` file, implement the type `Currency` as `Balances`
impl pallet_example for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
}

