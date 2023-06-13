//! How to create a view function like Solidity?
//!
//! So, w.r.t. substrate pallets, it is useful to
//! have functions in pallet (say A) that can be called to get data
//! from another pallet (say B). This is similar to Solidity's view functions.
//!
//! This doesn't require any gas fee.
//!
//! We just need to import pallet A into pallet B's `Cargo.toml` file.
//!
//! And then we can use that.
//!
//! Reference: `Timestamp` pallet has that `view` function - `get()` directly
//! implemented for `Pallet<T>`
//!
//! TODO: How to call the function from outside?
