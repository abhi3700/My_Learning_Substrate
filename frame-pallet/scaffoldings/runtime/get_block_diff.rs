//! Get block difference
//! 
//! Here, we get the block difference between the current block number and the block number when the last time amount staked.
//! 
//! This is used to calculate the interest amount for the staking duration.


// Here, `old_block_num` is of type `T::BlockNumber` and is the block number when the last time amount staked.
let block_difference = <frame_system::Pallet<T>>::block_number()
    .checked_sub(&old_block_num)
    .ok_or(Error::<T>::ArithmeticUnderflow)?;
