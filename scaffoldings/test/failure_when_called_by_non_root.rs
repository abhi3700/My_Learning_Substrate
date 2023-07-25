//! Test failure when the caller is not the root.

use sp_runtime::DispatchError;

#[test]
fn others_cant_set_fd_interest_rate() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Bank::set_fd_interest_rate(RuntimeOrigin::signed(ALICE), 800_000, 100_000),
            DispatchError::BadOrigin
        );
    });
}
