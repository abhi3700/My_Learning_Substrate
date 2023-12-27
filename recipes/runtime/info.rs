//! https://paritytech.github.io/substrate/master/frame_support/attr.pallet.html#pallet-struct-placeholder-palletpallet-mandatory

// Define pallet info inside the pallet struct (this is mandatory) which is used inside `mod <pallet-name>`

mod pallet {
    #[pallet::pallet]
    pub struct Pallet<T>(_);
}
