//! Balance type to u32 type conversion & viceversa.
mod pallet {
    impl<T: Config> for Pallet<T> {
        //function to convert balance to u32
        pub fn balance_to_u32(input: BalanceOf<T>) -> Option<u32> {
            TryInto::<u32>::try_into(input).ok()
        }

        // NOTE: prefer this for arithmetic operations to avoid truncation
        pub fn u32_to_balance(input: u32) -> Option<BalanceOf<T>> {
            TryInto::<BalanceOf<T>>::try_into(input).ok()
        }
    }
}

