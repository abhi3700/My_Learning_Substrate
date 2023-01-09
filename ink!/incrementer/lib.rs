#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Incrementer {
        /// Stores a single `bool` value on the storage.
        value: i32,
    }

    #[ink(event)]
    pub struct Incremented {
        old_value: i32,
        new_value: i32,
    }

    impl Incrementer {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_num: i32) -> Self {
            Self { value: init_num }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(32)
        }

        #[ink(message)]
        pub fn increment(&mut self, by: i32) {
            let old_value = self.value;
            let new_value = old_value + by;
            self.value = new_value;
            self.env().emit_event(Incremented {
                old_value,
                new_value,
            })
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> i32 {
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
            let incr = Incrementer::default();
            assert_eq!(incr.get(), 32);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn new_works() {
            let incr = Incrementer::new(45);
            assert_eq!(incr.get(), 45);
        }

        #[ink::test]
        fn increment_works() {
            let mut incr = Incrementer::default();
            assert_eq!(incr.get(), 32);
            incr.increment(45);
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            // println!("{:?}", &emitted_events[0]); // TODO: Debug trait not implemented. Issue raised: https://github.com/paritytech/ink/issues/1577
            assert_eq!(1, emitted_events.len());

            assert_eq!(incr.get(), 77);
            incr.increment(-56);
            assert_eq!(incr.get(), 21);
        }
    }
}
