//! get storage value from inside a dispatchable function

// inside the dispatchable

fn set_value(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
    let _ = ensure_signed(origin)?;

    // this one. get the value from storage
    // M-1
    let val = <SingleValue<T>>::get();
    // M-2
    let val = Self::single_value();

    Self::deposit_event(Event::Got(val));
    Ok(().into())
}
