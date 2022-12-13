//! File: https://github.com/JoshOrndorff/recipes/blob/master/pallets/hello-substrate/src/lib.rs

// Import the necessary dependencies from the FRAME pallet template, but don't use Rust std lib as the output is WASM
// Substrate runtimes are compiled to both Web Assembly and a regular native binary, and do not have access to rust's standard library. 
// Source: https://substrate.recipes/runtime-printing.html#no-std
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{debug, decl_module, dispatch::DispatchResult};
use frame_system::ensure_signed;
use sp_runtime::print;

mod pallet_hello {

    // Config required by the pallet
    pub trait Config::frame_system::Config {}

    // Each pallet must have a Config trait. Although it is empty
    // use `decl_module!` macro to define dispatchable calls i.e. external (extrinsic) txns outside the chain.
    decl_module! {
        pub struct Module<T: Config> for enum Call where origin: T::Origin {
            // this weight can be adjusted for the runtime i.e. the amount of gas to be consumed for this function execution
            #[weight = 10_000]
            fn say_hello() -> DispatchResult {
                // ensure that the caller is a regular keypair account
                // The ensure_signed function, used at the beginning, 
                // can return an error if the call was not from a signed origin.
                let caller = ensure_signed(origin)?;

                print("Hello, world!");
                debug::info!("called by {:?}", caller);

                // indicate the call succeeded.
                Ok(())
            }
        }
    }
}


// TODO:
// - try the folder on a separate repo & see if it works fine