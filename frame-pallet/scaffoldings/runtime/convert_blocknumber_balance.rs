//! Conversion from block number to balance.
//! 
//! 1. Just need to use the `where` clause to specify the trait bounds.
//! 2. Add `.into()` to convert from one type to another.

// `$ cargo add sp-runtime -p pallet-bank --no-default-features` at the node-template repo root.
use sp_runtime::traits::{CheckedDiv, CheckedMul, CheckedSub, Zero};

#[pallet::call]
impl<T: Config> Pallet<T> 
// NOTE: Here, `where` clause is required for converting from `BlockNumber` to `Balance`
// automatically suggested from compiler.
where
		<<T as pallet::Config>::MyCurrency as Currency<
			<T as frame_system::Config>::AccountId,
		>>::Balance: From<<T as frame_system::Config>::BlockNumber>, {
	
	#[pallet::call_index(4)]
	#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
	pub fn close_fd(origin: OriginFor<T>, id: u16) -> DispatchResult {
		//===snip===

		let interest = amount
			.checked_mul(&interest_rate.into())
			.and_then(|v| v.checked_mul(&block_difference.into()))	// NOTE: Here, `into()` is required for converting from `BlockNumber` to `Balance`
			.and_then(|v| v.checked_div(&block_duration.into()))
			.and_then(|v| v.checked_div(&scaling_factor.into()))
			.ok_or("Interest calculation failed")?;

		//===snip===

	}

}
