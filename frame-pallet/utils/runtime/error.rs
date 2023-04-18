//! Error:
//! - define
//! - throw

#[frame_support::pallet]
mod pallet {
    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// The string can't initiate with 'Hello'
        HelloPrefixed,
    }

    #[pallet::call_index(1)]
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn say_any(origin: OriginFor<T>, wish: String) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // check for the error
        if wish.starts_with("hello") {
            return Err(Error::<T>::HelloPrefixed.into());
        }

        print("Says Anything");

        // Emits an event
        Self::deposit_event(Event::SomeoneSaysAny { wish, who });

        Ok(())
    }
}
