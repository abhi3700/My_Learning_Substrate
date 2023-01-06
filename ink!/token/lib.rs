#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod token {

    use ink_storage::Mapping;

    /// The ERC-20 result type
    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Token {
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

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: Option<AccountId>,
        #[ink(topic)]
        spender: Option<AccountId>,
        value: Balance,
    }

    /// The ERC-20 error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
    }

    impl Token {
        #[ink(constructor)]
        pub fn new() -> Self {
            todo!()
        }

        // Getter

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Balance {
            self.balance_of_impl(&account)
        }

        #[inline]
        pub fn balance_of_impl(&self, account: &AccountId) -> Balance {
            self.balances.get(&account).unwrap_or_default()
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn allowance_of(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_impl(&owner, &spender)
        }

        #[inline]
        fn allowance_impl(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        // Setters

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            self.transfer_from_to(&caller, &to, value)?;

            Ok(())
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            self.allowances.insert((caller, spender), &value);

            self.env().emit_event(Approval {
                owner: Some(caller),
                spender: Some(spender),
                value: value,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let allowance = self.allowance_of(from, to);
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }

            self.allowances.insert((from, to), &(allowance - value));
            self.transfer_from_to(&from, &to, value)?;
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value: value,
            });

            Ok(())
        }

        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let balance_from = self.balance_of_impl(&from);
            if balance_from < value {
                return Err(Error::InsufficientBalance);
            }

            let balance_to = self.balance_of_impl(&to);
            self.balances.insert(from, &(balance_from - value));
            self.balances.insert(to, &(balance_to + value));

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value: value,
            });

            Ok(())
        }
    }
}
