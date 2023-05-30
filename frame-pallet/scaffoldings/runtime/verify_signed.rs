//! Get who is making the call here:
let who = ensure_signed(origin)?;

//! then check if the caller matches with the one in the storage:
ensure!(who == <WhoAmI>::get(), "You are not the owner of this account");