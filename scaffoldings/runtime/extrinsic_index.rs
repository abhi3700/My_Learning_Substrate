//! Get extrinsic index from the runtime.
//! Used in `remark` pallet in `paritytech/substrate` repo.
//! Code: https://github.com/paritytech/substrate/tree/master/frame/remark

let extrinsic_index = <frame_system::Pallet<T>>::extrinsic_index()
    .ok_or_else(|| Error::<T>::BadContext)?;
