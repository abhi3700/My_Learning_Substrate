//! Get who is making the call here:
let who = ensure_signed(origin)?;

//! then check if the caller matches with the required user i.e. owner 
//! or some other user in the storage.
//! Here, we need to define the `Owner` StorageValue in the pallet.
ensure!(who == <Owner<T>>::get(), "You are not the owner of this account");