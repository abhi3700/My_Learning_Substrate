//! define pallet storage for using at the consensus level

#[frame_support::pallet]
mod pallet {

    /// ## `StorageValue` that can be get/set/take/kill
    /// ### Usage:
    /// 1. Any type - primitive, derived or custom. Please ensure the trait bounds, otherwise you may get error.
    /// Define a storage value with the name `Value` of type `bool`
    #[pallet::storage]
    #[pallet::getter(fn value)] // here, the getter function name for this storage value is `value()`
    type Value<T> = StorageValue<_, bool>
    /// 
    /// `<Value<T>>::put(true);`: set the value to `true`
    /// `<Value<T>>::get();`: get the value
    /// `<Value<T>>::take();`: take the value and set it to `None` i.e. clear the storage
    /// `<Value<T>>::kill();`: kill the storage i.e. remove the storage from the storage trie
    /// ------------------------------------
    /// 2. A list of item i.e. fixed size array
    /// The one config trait which includes the max value of 
	#[pallet::config]
	pub trait Config: frame_system::Config + Get<u32> {
		/// A constant for the max size of owners
		#[pallet::constant]
		type MaxOwner: Get<u32>;

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn array)]
	// NOTE: Had to add trait bound for T with `Config`
	/// List of owners
	pub type SomeArray<T: Config> = StorageValue<_, BoundedVec<T::AccountId, T::MaxOwner>>;
    /// 
    /// For functions, refer this: https://docs.rs/sp-runtime/latest/sp_runtime/bounded_vec/struct.BoundedVec.html
    /// ------------------------------------
    /// 3. A vector of item i.e. dynamic size array
	#[pallet::storage]
	#[pallet::getter(fn land_owners)]
	// NOTE: Had to add trait bound for T with `Config`
	/// List of owners
	pub type LandOwners<T: Config> = StorageValue<_, Vec<T::AccountId>>;
    /// For functions, refer this: https://paritytech.github.io/substrate/master/frame_support/dispatch/struct.Vec.html
    /// 
    /// ====================================
    /// ## `StorageMap` that can be get/set/take/kill
    /// 
}