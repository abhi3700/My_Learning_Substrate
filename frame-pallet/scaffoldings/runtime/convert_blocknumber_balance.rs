//! Conversion from block number to balance and viceversa.
//! 
//! M-1: is used in `pallet-bank`: https://crates.io/crates/pallet-bank
//! 
//! 

//====M-1 ✅=====
mod pallet {
	impl<T: Config> for Pallet<T> {
		// Works in runtime and pallet.
		// No need to add `where` clause. Just add functions to convert from one type to another using From, Into, TryFrom, TryInto traits.
		//function to convert `Balance` to `T::BlockNumber`
		fn balance_to_blocknum(input: BalanceOf<T>) -> Option<T::BlockNumber> {
			TryInto::<T::BlockNumber>::try_into(input).ok()
		}

		//function to convert `T::BlockNumber` to `Balance`
		// NOTE: prefer this to avoid truncating during arithmetic operations
		fn blocknum_to_balance(input: T::BlockNumber) -> Option<BalanceOf<T>> {
			TryInto::<BalanceOf<T>>::try_into(input).ok()
		}
	}
}



//====M-2 ❌=====
/// 1. Just need to use the `where` clause to specify the trait bounds.
/// 2. Add `.into()` to convert from one type to another.
/// Works inside a pallet. But, not in the runtime.
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
