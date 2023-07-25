//! ## Call `Error` enum of other pallets
//!
//! Used inside `assert_noop!` macro to check if the error of
//! `pallet_balances` (for instance) is thrown.
//!
//! Here, the sequence of error defined in enum `Error` of `pallet_balances`
//! has to be matched i.e. LHS == RHS.
//!
//! `pallet_balances` is used as dependency in current pallet's `Cargo.toml`.
//!
//! NOTE: Here, please observe that `Balances` & `pallet_balances` is pointing to the same crate.
//! but differently used for different purposes - calling function, calling error.

/// Here,
/// 🧍 -> lock 10_000 (= free)
#[test]
fn success_when_locking_all_amt() {
    new_test_ext().execute_with(|| {
        assert_eq!(Balances::free_balance(1), 10_000);
        assert_ok!(LockableCurrency::lock_capital(
            RuntimeOrigin::signed(1),
            10_000
        ));
        System::assert_last_event(
            Event::Locked {
                user: 1,
                amount: 10_000,
            }
            .into(),
        );
        assert_eq!(Balances::free_balance(1), 10_000); // free_balance is still 10_000
        assert_noop!(
            Balances::transfer(RuntimeOrigin::signed(1), 2, 10), // transfer some
            pallet_balances::Error::<Test, _>::LiquidityRestrictions
        );
    });
}
