/*
    Add default values for storage struct via adding `Default` trait using `derive` macro
*/

// E.g. for token
#[ink(storage)]
#[derive(Default)]
pub struct Erc20 {
    /// Total token supply.
    total_supply: Balance,
    /// Mapping from owner to number of owned token.
    balances: Mapping<AccountId, Balance>,
    /// Mapping of the token amount which an account is allowed to withdraw
    /// from another account.
    allowances: Mapping<(AccountId, AccountId), Balance>,
}
