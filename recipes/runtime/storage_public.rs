//! Other pallets can access the storage of a pallet
//! on per block basis. This is done by attaching this attribute
//! macro `#[pallet::whitelist_storage]` to the storage item.
//!
//! Source: https://github.com/paritytech/substrate/blob/cbf8939f25bb7c8e2247a0136589f31e9b99aa4c/frame/support/procedural/src/lib.rs#L1548-L1569
//! Code inside file: `frame/support/procedural/src/lib.rs`

/// The optional attribute `#[pallet::whitelist_storage]` will declare the
/// storage as whitelisted from benchmarking. Doing so will exclude reads of
/// that value's storage key from counting towards weight calculations during
/// benchmarking.
///
/// This attribute should only be attached to storages that are known to be
/// read/used in every block. This will result in a more accurate benchmarking weight.
///
/// ### Example
/// ```ignore
/// #[pallet::storage]
/// #[pallet::whitelist_storage]
/// pub(super) type Number<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;
/// ```
///
/// NOTE: As with all `pallet::*` attributes, this one _must_ be written as
/// `#[pallet::whitelist_storage]` and can only be placed inside a `pallet` module in order for
/// it to work properly.
#[proc_macro_attribute]
pub fn whitelist_storage(_: TokenStream, _: TokenStream) -> TokenStream {
    pallet_macro_stub()
}
