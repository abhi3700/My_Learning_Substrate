//! Events
//! - define
//! - emit

#[frame_support::pallet]
mod pallet {

    // Define
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomeoneSaysHello {
            who: T::AccountId,
        },
        SomeoneSaysAny {
            wish: String,
            who: T::AccountId,
        },
    }

    #[pallet::call_index(1)]
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn say_any(origin: OriginFor<T>, wish: String) -> DispatchResult {
        let who = ensure_signed(origin)?;

        if wish.starts_with("hello") {
            return Err(Error::<T>::HelloPrefixed.into());
        }

        print("Says Anything");

        // Emits an event
        Self::deposit_event(Event::SomeoneSaysAny { wish, who });

        Ok(())
    }
}

// Emit
