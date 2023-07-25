// imports
use frame_support::{
    log,
    pallet_prelude::*,
    sp_runtime::{
        traits::{CheckedDiv, CheckedMul, CheckedSub, One, Zero},
        DispatchError,
    },
    traits::{
        Currency, ExistenceRequirement::AllowDeath, LockIdentifier, LockableCurrency,
        NamedReservableCurrency, ReservableCurrency, WithdrawReasons,
    },
    Blake2_128Concat,
};

// for permill, perbill, perquintill, perpercent.
pub use sp_arithmetic::PerThing;
