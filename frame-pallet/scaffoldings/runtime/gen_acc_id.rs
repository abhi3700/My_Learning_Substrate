//! It is used to generate some accounts for development purpose.
//! It is used inside `testnet_genesis()` function of `substrate-node-template/node/src/chain_spec.rs` file

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}


// Usage
get_account_id_from_seed::<sr25519::Public>("Alice")