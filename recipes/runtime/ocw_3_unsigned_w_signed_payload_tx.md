---
title: OCW Unsigned w Signed Payload Transaction
---

**Inside the pallet Config trait:**

<!-- TODO: -->

```rust
#[frame_support::pallet]
pub mod pallet {
	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	}
}
```

**Inside the pallet hooks trait implementation:**

<!-- TODO: -->
