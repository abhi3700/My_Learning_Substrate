---
title: Define & Use Balance Type in `mock.rs` and `tests.rs`
---

```rust
//! File: mock.rs

// Balances Pallet
impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<100>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type HoldIdentifier = ();
	type MaxHolds = ();
}

// NOTE: Here, Balance type of Balances pallet is annotated as `Balance` type of your pallet's associated types.
parameter_types! {
	pub const MinFDAmount: <Test as pallet_balances::Config>::Balance = 50;
	pub const MaxFDAmount: <Test as pallet_balances::Config>::Balance = 200_000;
	pub const MinLockValue: <Test as pallet_balances::Config>::Balance = 20;
	pub const MaxLockValue: <Test as pallet_balances::Config>::Balance = 100_000;
	pub const MaxFDMaturityPeriod: u32 = ONE_YEAR;
}
```

```rust
//! File: tests.rs

use crate::mock::*;

// NOTE: Here, Balance type of Balances pallet is annotated as `Balance` type for the Principal Amount.
const PRINCIPAL_AMOUNT: Balance = 100;
```
