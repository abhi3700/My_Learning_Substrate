//! How to check if an account is reaped.
//!
//! Following are the steps:
//! 1. Ensure there is some free_balance of the account.
//! 2. transfer all the free_balance to another account (say bob).
//! 3. Ensure the providers, consumers of the account are empty.
//! 4. Now, ensure the account key is NOT present in the storage.

#[test]
fn account_should_be_reaped() {
    ExtBuilder::default()
        .existential_deposit(1)
        .monied(true)
        .build_and_execute_with(|| {
            assert_eq!(Balances::free_balance(1), 10);
            assert_ok!(<Balances as Currency<_>>::transfer(&1, &2, 10, AllowDeath));
            assert_eq!(System::providers(&1), 0);
            assert_eq!(System::consumers(&1), 0);
            // Check that the account is dead.
            assert!(!frame_system::Account::<Test>::contains_key(&1));
        });
}
