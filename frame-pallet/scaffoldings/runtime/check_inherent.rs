//! How to check the call origin of an extrinsic has no signature done i.e. transaction is inherent.
//!
//! Inherent i.e. unsigned

use frame_support::ensure_none;

// inside the dispatchable
fn set_time(origin: OriginFor<T>, now: T::Moment) -> DispatchResultWithPostInfo {
    ensure_none(origin)?;
    <Now<T>>::put(now);
    Ok(().into())
}
