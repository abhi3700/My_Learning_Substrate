//! BoundedVec is a collection for substrate.
//!
//! It's defined as:
//! ```
//! struct BoundedVec<T, S>(Vec<T>);
//! ```
//!
//! where,
//! - T: type of the element
//! - S: A type that implements Get<u32> trait of sp_core, representing the vec cap.

use frame_support::BoundedVec;
use sp_core::Get;

pub struct MaxLen;
impl Get<u32> for MaxLen {
    fn get() -> u32 {
        5
    }
}

fn main() -> eyre::Result<()> {
    let mut v1 = BoundedVec::<u32, MaxLen>::default();

    v1.try_push(1).map_err(|_| eyre::eyre!("within bounds"))?;
    v1.try_push(2).map_err(|_| eyre::eyre!("within bounds"))?;
    v1.try_push(3).map_err(|_| eyre::eyre!("within bounds"))?;
    v1.try_push(4).map_err(|_| eyre::eyre!("within bounds"))?;
    v1.try_push(5).map_err(|_| eyre::eyre!("within bounds"))?;

    // ❌ Fails as max. len is 5.
    assert!(v1.try_push(6).is_err());

    println!("{:?}", v1);

    Ok(())
}
