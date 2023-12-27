---
title: PerThing - Percent, Permill, Perbill
---

PerThing is a _trait_ which allows to represent a per-thing storage.
where thing: bill, mill, etc.

`Permill` is a _struct_ type used in Substrate to represent a fraction of a whole (1_000_000) in millionths. Here are some examples of `Permill` values:

1. `Permill::from_percent(50)` represents 50%, or 500_000 parts per million.
2. `Permill::from_parts(1_000_000)` represents 100%, or 1_000_000 parts per million.
3. `Permill::from_parts(500_000)` represents 50%, or 500_000 parts per million.
4. `Permill::from_parts(0)` represents 0%, or 0 parts per million.

---

Deconstruct `Permill` into its parts - Nr of parts per million:

```rust
// let ir = Permill::from_parts(20000);
let ir = Permill::from_percent(2);
println!("ir: {:?}", ir.deconstruct()); // ir: 20000
```

---

Addition, Multiplication, Division for Permill types:

```rust
// division for Permill
let x: Permill = interest*rate.saturating_div(pc(n * 100), Rounding::NearestPrefUp);
// Addition for Permill
let z: Permill = x + One::one();
// Multiplication for Permill; Also used for converting to u32
let z: u32 = z * 1u32;
```

---

Short-hand for `Permill::from_percent()`. Similarly, we can do this for other functions like `Permill::from_parts()`, etc.

```rust
let pc: impl Fn(u32) -> Permill = |x: u32| Permill::from_percent(x);

// usage:
pc(50) // 50%
```

---

You can use these values in test cases like this:

```rust
#[cfg(test)]
mod tests {
	use super::*;
	use sp_runtime::Permill;

	#[test]
	fn test_calculate_investment_score() {
		let maturity_amount = FixedU128::from(1000);
		let difficulty_factor = FixedU128::from(100);

		// Test with 50% difficulty
		let difficulty = Permill::from_percent(50);
		let result = calculate_investment_score(maturity_amount, difficulty_factor * difficulty.into());
		assert!(result.is_some());
		// Add assertions based on your expected result

		// Test with 100% difficulty
		let difficulty = Permill::from_parts(1_000_000);
		let result = calculate_investment_score(maturity_amount, difficulty_factor * difficulty.into());
		assert!(result.is_some());
		// Add assertions based on your expected result

		// Test with 0% difficulty
		let difficulty = Permill::from_parts(0);
		let result = calculate_investment_score(maturity_amount, difficulty_factor * difficulty.into());
		assert!(result.is_some());
		// Add assertions based on your expected result
	}
}
```

In this example 🔝, `calculate_investment_score` is your function for calculating the investment score.
The test cases cover different values of the difficulty factor, represented as a percentage of the total difficulty factor. You would need to replace the comments with assertions based on your expected results.

Write pallet extrinsic like this:

```rust
    #[pallet:call]
    impl<T: Config> Pallet<T> {
    	#[pallet::call_index(0)]
    	#[pallet::weight(T::WeightInfo::do_something())]
    	pub fn do_something(
    		origin: OriginFor<T>,
    		staked_amount: u64,
    		annual_interest_rate: Permill,  // NOTE: this is the line matters
    		duration_in_years: u32,
    	) -> DispatchResult {
    		let who = ensure_signed(origin)?;

    		// ensure positive interest
    		ensure!(annual_interest_rate > Permill::zero(), Error::<T>::ZeroFDInterestRate);
    		// ensure!(annual_interest_rate > Permill::one(), Error::<T>::ZeroFDInterestRate); // or use > 1

    		let something: u64 =
    			Self::calculate_interest(staked_amount, annual_interest_rate, duration_in_years)
    				.ok_or(Error::<T>::StorageOverflow)?;

    		// Update storage.
    		<Something<T>>::put(something);

    		// Emit an event.
    		Self::deposit_event(Event::SomethingStored { something, who });
    		// Return a successful DispatchResultWithPostInfo
    		Ok(())
    	}

    }

    impl<T: Config> Pallet<T> {
    	fn calculate_interest(
    		staked_amount: u64,
    		annual_interest_rate: Permill,      // this is the line matters
    		duration_in_years: u32,
    	) -> Option<u64> {
            // NOTE: here, it is directly multiplied by staked_amount w/o any `checked_*` functions
    		let annual_interest = annual_interest_rate * staked_amount;
    		let interest = annual_interest.checked_mul(duration_in_years as u64)?;
    		interest.checked_div(365)
    	}
    }
```
