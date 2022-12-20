#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod counter {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Counter {
        /// Stores a single `u8` value on the storage.
        value: u8,
    }

    impl Counter {
        /// Constructor that initializes the `u8` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: u8) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `u8` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one increments the value of the stored `u8` by 1
        #[ink(message)]
        pub fn increment(&mut self) {
            self.value += 1;
        }

        /// A message that can be called on instantiated contracts.
        /// This one decrements the value of the stored `u8` by 1
        #[ink(message)]
        pub fn decrement(&mut self) {
            self.value -= 1;
        }

        /// Simply returns the current value of our `u8`.
        #[ink(message)]
        pub fn get_val(&self) -> u8 {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let counter = Counter::default();
            assert_eq!(counter.get_val(), 0);
        }

        #[ink::test]
        fn new_works() {
            let counter = Counter::new(5);
            assert_eq!(counter.get_val(), 5);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn increment_works() {
            let mut counter = Counter::new(5);
            assert_eq!(counter.get_val(), 5);
            counter.increment();
            assert_eq!(counter.get_val(), 6);
        }

        #[ink::test]
        fn decrement_works() {
            let mut counter = Counter::new(5);
            assert_eq!(counter.get_val(), 5);
            counter.decrement();
            assert_eq!(counter.get_val(), 4);
        }
    }
}
