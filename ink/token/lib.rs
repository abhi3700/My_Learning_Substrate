#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod token {

    use ink_storage::Mapping;

    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Token {
        /// Stores diff. values in storage
        name: String,
        symbol: String,
        decimal: u8,
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event emitted for the approval i.e. spender is able to withdraw upto the amount from owner.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
    }

    impl Token {
        /// Constructor that initializes the token with name, symbol, .
        #[ink(constructor)]
        pub fn new(name: String, symbol: String, decimal: u8, total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            balances.insert(Self::env().caller(), &total_supply);
            Self {
                name,
                symbol,
                decimal,
                total_supply,
                balances,
                allowances: Default::default(),
            }
        }

        /// allowance function to get the value for getting the balance value for (owner, spender)
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_impl(&owner, &spender)
        }

        /// Wasm optimized function
        #[inline]
        fn allowance_impl(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        /// Simply returns the current value of our balance of an account
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Balance {
            self.balance_of_impl(&account)
        }

        #[inline]
        pub fn balance_of_impl(&self, account: &AccountId) -> Balance {
            self.balances.get(&account).unwrap_or_default()
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            // self.env().
            self.allowances.insert((&owner, &spender), &value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)?;

            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            // todo!()
            // todo: found a bug here: https://github.com/paritytech/ink/blob/master/examples/erc20/lib.rs#L176
            // check approval first
            let approved_value = self.allowance_impl(&from, &to);
            if approved_value >= value {
                return Err(Error::InsufficientAllowance);
            }

            self.transfer_from_to(&from, &to, value)?;
            self.allowances
                .insert((&from, &to), &(approved_value - value));
            Ok(())
        }

        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let frm_balance = self.balance_of_impl(from);
            let to_balance = self.balance_of_impl(to);

            if frm_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(&from, &(frm_balance - value));
            self.balances.insert(&to, &(to_balance + value));

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        #[ignore = "reason"]
        fn test_transfer() {
            todo!()
        }
    }
}
