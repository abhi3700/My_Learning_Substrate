//! Define test accounts for testing.
//!
//! File: src/mock.rs

// define test accounts
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const CHARLIE: u64 = 3;
pub const DAVE: u64 = 4;
pub const TREASURY: u64 = 100;

// Then, use them in assigning balances to accounts
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(ALICE, 10_000),
			(BOB, 20_000),
			(CHARLIE, 30_000),
			(DAVE, 40_000),
			(TREASURY, 1_000_000),
		],
	}

