//! define pallet storage for using at the consensus level

#[frame_support::pallet]
mod pallet {

    #[pallet::storage]
    #[pallet::getter(fn value)] // here, the getter function name for this storage value is `value()`
    type Value<T> = StorageValue<_, bool>
}