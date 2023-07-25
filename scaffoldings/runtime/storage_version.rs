//! Set the storage version of a pallet.
//! Then during storage model upgrade change the version of pallet.

mod pallet {
    use frame_support::pallet_prelude::StorageVersion;
    use frame_support::traits::StorageVersion as _;

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);
}
