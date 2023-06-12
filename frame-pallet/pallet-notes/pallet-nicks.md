# Nicks Pallet

## Overview

Allows anyone to set nick name for their account id in return of some currency amount to be reserved.

## Notes

- Usage of `name` variable type as `String`. [Details](../scaffoldings/runtime/extrinsic_string_type.rs)
- Constants:
  - `MaxLength`: Maximum length of the name.
  - `MinLength`: Maximum length of the name.
  - `ReservationFee`: Maximum length of the name.
- Usage of shorthand while extracting values after checking the database value. [Line](https://github.com/paritytech/substrate/blob/master/frame/nicks/src/lib.rs#L142)

  ```rust
  let deposit = if let Some((_, deposit)) = <NameOf<T>>::get(&sender) {
      Self::deposit_event(Event::<T>::NameChanged { who: sender.clone() });
      deposit
  } else {
      let deposit = T::ReservationFee::get();
      T::Currency::reserve(&sender, deposit)?;
      Self::deposit_event(Event::<T>::NameSet { who: sender.clone() });
      deposit
  };
  ```

- StorageMap with key: `T::AccountId` & value: `(BoundedVec<u8, T::MaxLength>, BalanceOf<T>)`. [Code](https://github.com/paritytech/substrate/blob/master/frame/nicks/src/lib.rs#L114)

## References

- [Pallet Nicks Github](https://github.com/paritytech/substrate/tree/master/frame/nicks)
