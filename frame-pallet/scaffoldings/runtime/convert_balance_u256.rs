//! To/From Balance to U256 conversion.
//!
//! Source: https://vscode.dev/github/paritytech/substrate/blob/master/bin/node/runtime/src/lib.rs#L794-L807

use frame_support::sp_runtime::traits::Convert;
pub struct BalanceToU256;
impl Convert<Balance, sp_core::U256> for BalanceToU256 {
    fn convert(balance: Balance) -> sp_core::U256 {
        // sp_core::U256::from(balance) // using From trait
        balance.into() // using Into trait
    }
}
pub struct U256ToBalance;
impl Convert<sp_core::U256, Balance> for U256ToBalance {
    fn convert(n: sp_core::U256) -> Balance {
        // here U256 to U128 conversion is happening. So, if the value is greater than U128::max_value(),
        // then it will return U128::max_value()
        n.try_into().unwrap_or(Balance::max_value())
    }
}
