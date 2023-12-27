//! How to treat a string as a type inside dispatchable
//! It's very uncommon in Substrate Rust here to use a string as a type.
//!
//! In substrate, we have to give the `name` variable as bytes i.e., `Vec<u8>` type for:
//!   - dispatchable as local variable type
//!
//! We have to store the field `name` as `BoundedVec` type i.e. `BoundedVec<u8, T::MaxStringLength>`
//! Declare the max, min string length as constants in the Config trait of the pallet like this:
//!
//! ```rust
//! pub trait Config: frame_system::Config {
//!     #[pallet::constant]
//!     type MaxStringLength: Get<u32>;
//!     #[pallet::constant]
//!     type MinStringLength: Get<u32>;
//! }
//! ```

// ===== inside pallet `src/lib.rs` file

// need to import this for `Vec<u8>` type
use scale_info::prelude::vec::Vec;

/// A type for a single proposal.
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, RuntimeDebug, Default, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Proposal<T: Config> {
    proposer: T::AccountId,
    name: BoundedVec<u8, T::MaxProposalLength>,
    vote_count: u32,
    // TODO: Research for adding a timestamp type here.
    // Reference: https://stackoverflow.com/questions/68262293/substrate-frame-v2-how-to-use-pallet-timestamp
    vote_start_timestamp: Option<T::BlockNumber>,
    vote_end_timestamp: Option<T::BlockNumber>,
}

// This is how to use a string as a type inside dispatchable
#[pallet::call_index(2)]
#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
pub fn set_name(origin: OriginFor<T>, name: Vec<u8>) -> DispatchResult {
    let bounded_name: BoundedVec<_, _> = name.try_into().map_err(|_| Error::<T>::TooLong)?;
    ensure!(
        bounded_name.len() >= T::MinLength::get() as usize,
        Error::<T>::TooShort
    );
}
