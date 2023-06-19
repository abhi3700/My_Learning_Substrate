//! Define all the types - primitive, custom

type AccountOf<T> = <T as frame_system::Config>::AccountId;

/// balance type used for any amount of currency say....
/// Usage: `fn stake(origin, amount: BalanceOf<T>)`
type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// hash type used for any hash
/// Usage: `fn verify(origin, hash: HashOf<T>)`
type HashOf<T> = <T as frame_system::Config>::Hash;
