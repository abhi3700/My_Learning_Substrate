//! # Helper function
//!
//! Sometimes a dispatchable function inside a pallet reuses logic
//! that is common to other dispatchables. In this case, it's useful
//! to refactor this logic into its own private function. At other times,
//! dispatchable functions get increasingly difficult to read as the amount
//! of code increases to perform various checks within the dispatchable.
//! In both instances, using helper functions that cannot be accessed from
//! outside the pallet are a useful tool to optimize for code readability and
//! reusability. In this guide, we'll see how to create an adder helper that
//! checks for arithmetic overflow and can be reused in any dispatchable.
//!
//! ## References
//! - https://docs.substrate.io/reference/how-to-guides/basics/use-helper-functions/

impl<T: Config> Pallet<T> {
    fn _adder(num1: u32, num2: u32) -> Result<u32, &'static str> {
        num1.checked_add(num2).ok_or("Overflow when adding")
    }
}

#[pallet::call]
// Extrinsics callable from outside the runtime.
impl<T: Config> Pallet<T> {
    #[pallet::weight(0)]
    fn add_value(origin: OriginFor<T>, val_to_add: u32) -> DispatchResultWithPostInfo {
        let _ = ensure_signed(origin)?;

        ensure!(
            val_to_add <= T::MaxAddend::get(),
            "value must be <= maximum add amount constant"
        );

        // previous value got
        let c_val = SingleValue::<T>::get();

        // checks for overflow when new value added
        let result = _adder(c_val, val_to_add);

        <SingleValue<T>>::put(result);
        Self::deposit_event(Event::Added(c_val, val_to_add, result));
        Ok(().into())
    }
}
